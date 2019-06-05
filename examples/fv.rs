extern crate lalrpop_lambda;

use lalrpop_lambda::lambda::ExpressionParser;

fn main() {
    let parser = ExpressionParser::new();

    dbg!(parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap().free_variables());
    dbg!(parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap().free_variables());
}
