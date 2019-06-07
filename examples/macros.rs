#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

fn main() {
    let ω = abs!{x.app!(x,x)};
    // Doesn't do what you might want.
    println!("{}", app!(x,ω));
    // But this does ;)
    println!("{} -> {}", app!({&ω},x), app!({&ω},x).normalize(false));

    println!();
    let id = λ!{x.x};
    print!("id: {}", id);
    let id = abs!{x.x};
    println!(" = {}", id);

    let zero = λ!{f.λ!{x.x}};
    println!("0: {}", &zero);
    let one = λ!{f.λ!{x.γ!(f, x)}};
    println!("1: {}", &one);
    let two = λ!{f.λ!{x.γ!(f, γ!(f, x))}};
    println!("2: {}", &two);
    let three = λ!{f.λ!{x.γ!(f, γ!(f, γ!(f, x)))}};
    println!("3: {}", &three);

    let succ = λ!{n.λ!{f.λ!{x.γ!(f, γ!(n, γ!(f, x)))}}};
    println!("succ: {}", &succ);
    let add = λ!{m.λ!{n.λ!{f.λ!{x.γ!(m, γ!(f, γ!(n, γ!(f, x))))}}}};
    println!("add:  {}", &add);

    println!();
    println!("(normalize (id one)) -> {}",
             app!({&id}, {&one}).normalize(false));

    println!("(add one one) -> {}",
             app!(app!({&add}, {&one}), {&one}));
    println!("(normalize (add one one)) -> {}",
             app!(app!({&add}, {&one}), {&one}).normalize(false));

    println!();
    println!("(succ one) -> {}",
             app!({&succ}, {&one}));
    println!("(normalize (succ one)) -> {}",
             app!({&succ}, {&one}).normalize(false));

    println!();
    println!("{}",
             abs!{f.abs!{x.
                 app!(var!(f),
                      app!(app!(abs!{f.abs!{x.app!(var!(f), var!(x))}},
                                var!(f)),
                           var!(x)))}}.normalize(false));
}
