#![feature(box_syntax, non_ascii_idents)]

#[macro_use]
extern crate lalrpop_lambda;

macro_rules! parse {
    ($expr:expr $(, $func:expr)?) => {{
        let e = ExpressionParser::new().parse($expr).unwrap();
        print!("{} parse-> {}", $expr, e);
        $(
            let e = $func(&e, &Strategy::Applicative(false));  // very funky.
            print!(" -> {}", e);
        )?
        println!("");
        e
    }}
}

fn main() {
    println!("ω = {}", λ!{x.γ!(x,x)});
    println!("Ω = {}", γ!(λ!{x.γ!(x,x)},λ!{x.γ!(x,x)}));
    println!("W = {}", λ!{f.λ!{x.γ!(γ!(f,x),x)}});
    println!("Y = {}", λ!{f.γ!(λ!{x.γ!(f,γ!(x,x))},λ!{x.γ!(f,γ!(x,x))})});
}
