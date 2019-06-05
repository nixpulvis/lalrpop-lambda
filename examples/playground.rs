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
    dbg!({
        let ω = parser.parse(r"λx.(x x)").unwrap();
        ω.to_s()
    });
    dbg!({
        let Ω = parser.parse(r"(λx.(x x)) (λx.(x x))").unwrap();
        &Ω.to_s()
    });
    dbg!({
        let W = parser.parse(r"λf.λx. f x x").unwrap();
        &W.to_s()
    });
    dbg!({
        let Y = parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))").unwrap();
        &Y.to_s()
    });

    let multi = parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap();
    dbg!(&multi.to_s());

    println!("\n\n\n");
    dbg!(parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap().normalize());

    println!("\n\n\n");
    dbg!(parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap().free_variables());

    println!("\n\n\n");
    dbg!(parser.parse(r"\f.\x.(f (x x) f (x x)) y").unwrap().normalize().to_s());

    dbg!(parser.parse(r"\x. f (x x)"));
    dbg!(parser.parse(r"\x. f x x"));
    dbg!(parser.parse(r"\f.(\x. f (x x)) (\x. f (x x))"));
    dbg!(parser.parse(r"(\x. (x x)) (\x. (x x))"));
}
