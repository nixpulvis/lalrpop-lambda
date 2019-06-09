#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;
use lalrpop_lambda::Expression;

macro_rules! resolve {
    ($expr:expr, $env:expr) => {
        println!("{} -r> {} -> {}",
                 $expr,
                 $expr.resolve($env),
                 $expr.resolve($env).normalize(false));
    }
}

fn main() {
    let env = map! {
        variable!(i) => abs!{x.x},
        variable!(n) => Expression::from(1),
        variable!(x) => var!(x),
    };

    println!("GLOBAL ENV");
    for (v, e) in &env {
        println!("{} := {}", v, e);
    }
    println!();

    resolve!(var!(i), &env);
    resolve!(var!(n), &env);
    resolve!(var!(x), &env);
    resolve!(var!(q), &env);
    resolve!(abs!{a.a}, &env);
    resolve!(abs!{a.n}, &env);
    resolve!(app!(a,n), &env);
    resolve!(app!(n,a), &env);
    resolve!(app!(n,n), &env);
    resolve!(app!(i,n), &env);
}
