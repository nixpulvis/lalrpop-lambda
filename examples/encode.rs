#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

use lalrpop_lambda::Expression;

fn main() {
    // // TODO: Move these to tests as we finalize.
    // dbg!(u64::from(var!(x)));
    // dbg!(u64::from(var!(f)));
    // dbg!(u64::from(app!(f,app!(f,x))));
    // dbg!(u64::from(abs!{f.app!(f,app!(f,x))}));
    // dbg!(u64::from(abs!{f.abs!{x.app!(f,app!(f,x))}}));

    let n = 0;
    let ln = Expression::from(n);
    let nn = u64::from(ln.clone());
    println!("{} -> {} -> {}", n, ln, nn);

    let n = 1;
    let ln = Expression::from(n);
    let nn = u64::from(ln.clone());
    println!("{} -> {} -> {}", n, ln, nn);

    let n = 5;
    let ln = Expression::from(n);
    let nn = u64::from(ln.clone());
    println!("{} -> {} -> {}", n, ln, nn);
}
