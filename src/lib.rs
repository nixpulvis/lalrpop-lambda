#![feature(box_syntax)]

#[macro_use]
extern crate lalrpop_util;

#[derive(Debug)]
pub enum Expression {
    Var(Variable),
    Abs(Abstraction),
    App(Application),
}

#[derive(Debug)]
pub struct Variable(pub String);

#[derive(Debug)]
pub struct Abstraction(pub Variable, pub Box<Expression>);

#[derive(Debug)]
pub struct Application(pub Box<Expression>, pub Box<Expression>);

lalrpop_mod!(pub lambda);

#[cfg(test)]
mod tests {
    use crate::lambda::ExpressionParser;

    #[test]
    fn variable() {
        assert!(ExpressionParser::new().parse(r"x").is_ok());
    }

    #[test]
    fn abstraction() {
        assert!(ExpressionParser::new().parse(r"\x.x").is_ok());
        assert!(ExpressionParser::new().parse(r"\x. x").is_ok());
        assert!(ExpressionParser::new().parse(r"\x.(x)").is_ok());
        assert!(ExpressionParser::new().parse(r"\x. (x)").is_ok());
    }

    #[test]
    fn application() {
        assert!(ExpressionParser::new().parse(r"x x").is_ok());
        assert!(ExpressionParser::new().parse(r"(x y)").is_ok());
        assert!(ExpressionParser::new().parse(r"(\x.x y)").is_ok());
    }
}
