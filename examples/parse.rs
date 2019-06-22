#![feature(non_ascii_idents)]
extern crate lalrpop_lambda;

use lalrpop_lambda::{Expression, Strategy};
use lalrpop_lambda::parse::ExpressionParser;

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
    parse!("x");
    parse!(r"\x.x");
    parse!(r"\x.y");
    parse!("x x");
    parse!("x y");

    // A type!
    parse!(r"\x:t.x x");

    println!();
    parse!(r"\\\x y.x y");
    parse!(r"\x y.x y");
    parse!(r"\\\");
    println!();

    println!();
    parse!(r"(\x.x) x", Expression::normalize);
    parse!(r"(\x.x) y", Expression::normalize);

    // Single β-reduction identity function.
    println!();
    parse!(r"\x.x a", Expression::normalize);
    parse!(r"(\x.x) a", Expression::normalize);

    // Partial application.
    println!();
    let norm = parse!(r"(\x.\y.x y) a", Expression::normalize);
    parse!(&format!("({}) b", norm), Expression::normalize);
    // Multiple (curried) β-reductions on an identity function.
    parse!(r"(\x.\y.x y) a b", Expression::normalize);

    println!();
    parse!(r"((\x.(\x.x x) a) b)", Expression::normalize);

    // Ω
    println!();
    parse!(r"\x.(x x) (\x.(x x))");
    parse!(r"(\x.(x x)) (\x.(x x))");
    // XXX: Blows the stack in our strategy.
    parse!(r"(\x.(x x)) (\x.(x x))", Expression::normalize);
}
