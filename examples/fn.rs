#![feature(box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

use lalrpop_lambda::Expression;

fn main() {
    let two = abs!{f.abs!{x.app!(var!(f), app!(var!(f), var!(x)))}};
    println!("{}", two(var!(x))(var!(x)));
    println!("{}", var!(x)(var!(y)));
    println!("{}", app!(var!(x),var!(y))(var!(z)));

    println!("{:?}", λ!{x.x}(1));

    let id: fn(u64) -> u64 = |x| x;
    println!("{}", Expression::from(id));
    let f = <fn(u64) -> u64>::from(abs!{x.x});
    println!("{}", Expression::from(f));
}
