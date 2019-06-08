#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

fn main() {
    // Both the short `λ!` macro, as well as the ASCII `abs!` macro.
    println!("{} = {}", λ!{x.x}, abs!{x.x});

    // Mix in rust bindings with lambda calculus macros using `{binding}`.
    let ω = abs!{x.app!(x,x)};
    // Doesn't do what you might want.
    println!("app!(x,ω): {}", app!(x,ω));
    // But this does ;)
    println!("app!({{&ω}},x): {} -> {}",
             app!({&ω},x),
             app!({&ω},x).normalize(false));

    // An empty church numeral.
    let zero = λ!{f.λ!{x.x}};
    println!("0: {}", &zero);

    // A single church numeral.
    let one = λ!{f.λ!{x.γ!(f, x)}};
    println!("1: {}", &one);

    // Identity application.
    let id = λ!{x.x};
    println!("(id one): {} -> {}",
             app!({&id}, {&one}),
             app!({&id}, {&one}).normalize(false));

    let two = abs!{f.abs!{x.app!(var!(f),
                                 app!(app!(abs!{f.abs!{x.app!(var!(f),
                                                              var!(x))}},
                                           var!(f)),var!(x)))}};
    println!("two: {} -> {}", two, two.normalize(false));
}
