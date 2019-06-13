#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

use std::collections::HashMap;
use lalrpop_lambda::Strategy;

macro_rules! resolve {
    ($expr:expr, $env:expr) => {
        println!("{} -r> {} -> {}",
                 $expr,
                 $expr.resolve($env),
                 $expr.resolve($env).normalize(&Strategy::Applicative(false)));
    }
}

fn main() {
    let mut env = HashMap::new();
    env.insert(variable!(i), abs!{x.x});
    env.insert(variable!(n), 1.into());
    env.insert(variable!(x), var!(x));
    for (v, e) in &env {
        println!("{} := {}", v, e);
    }
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
