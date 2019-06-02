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

    // Make the Y combinator.
    let ω = parser.parse(r"λx.(x x)");
    let Ω = parser.parse(r"(λx.(x x)) (λx.(x x))");
    let W = parser.parse(r"λf.λx. f x x");
    let Y = parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))");
    dbg!(ω);
    dbg!(Ω);
    dbg!(W);
    dbg!(Y);
}
