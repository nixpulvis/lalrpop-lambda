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
    dbg!(parser.parse(r"\x. f (x x)"));
    dbg!(parser.parse(r"\x. f x x"));
    dbg!(parser.parse(r"\f.(\x. f (x x)) (\x. f (x x))"));
}
