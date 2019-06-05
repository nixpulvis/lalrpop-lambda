#![feature(non_ascii_idents)]
extern crate lalrpop_lambda;

use lalrpop_lambda::parse::ExpressionParser;

fn main() {
    let parser = ExpressionParser::new();

    // Make the Y combinator.
    println!("ω = {}", parser.parse(r"λx.(x x)").unwrap());
    println!("Ω = {}", parser.parse(r"(λx.(x x)) (λx.(x x))").unwrap());
    println!("W = {}", parser.parse(r"λf.λx. f x x").unwrap());
    println!("Y = {}", parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))").unwrap());
}
