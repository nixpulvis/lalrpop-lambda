#![feature(non_ascii_idents)]
extern crate lalrpop_lambda;

use lalrpop_lambda::lambda::ExpressionParser;

fn main() {
    let parser = ExpressionParser::new();

    // Bad.
    dbg!(parser.parse(r"\x.\y.y \a.b"));
    dbg!(parser.parse(r"\x.\y.(y \a.b)"));

    // Good.
    dbg!(parser.parse(r"x"));
    dbg!(parser.parse(r"\x.x"));
    dbg!(parser.parse(r"\x.x y"));
    dbg!(parser.parse(r"\x.\y.y z"));

    // Identity function.
    let id = parser.parse(r"λx.x").unwrap();
    dbg!(&id.to_s());

    // Make the Y combinator.
    let ω = parser.parse(r"λx.(x x)").unwrap();
    dbg!(&ω.to_s());
    let Ω = parser.parse(r"(λx.(x x)) (λx.(x x))").unwrap();
    dbg!(&Ω.to_s());
    let W = parser.parse(r"λf.λx. f x x").unwrap();
    dbg!(&W.to_s());
    let Y = parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))").unwrap();
    dbg!(&Y.to_s());

    let multi = parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap();
    dbg!(&multi.to_s());

    println!("\n\n\n");
    dbg!(parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap().normalize());

    println!("\n\n\n");
    dbg!(parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap().free_variables());

    println!("\n\n\n");
    dbg!(parser.parse(r"\f.\x.(f (x x) f (x x)) y").unwrap().normalize().to_s());
}
