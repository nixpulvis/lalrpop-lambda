#![feature(box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

fn main() {
    let two = abs!{f.abs!{x.app!(var!(f), app!(var!(f), var!(x)))}};
    println!("{}", two(var!(x))(var!(x)));
    println!("{}", var!(x)(var!(y)));
    println!("{}", app!(var!(x),var!(y))(var!(z)));

    println!("{:?}", λ!{x.x}(1));
}
