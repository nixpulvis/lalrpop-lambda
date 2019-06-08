#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;
use lalrpop_lambda::Expression;

macro_rules! resolve {
    ($t:ty: $e:expr, $env:expr) => {{
        if let Some(r) = $e.resolve::<$t>($env) {
            println!("{} -r> {}", $e, r);
            r
        } else {
            println!("{}", $e);
            $e.into()
        }
    }}
}

fn main() {
    let env = map! {
        variable!(x) => var!(x),
        variable!(n) => Expression::from(1),
        variable!(i) => abs!{x.x},
    };

    println!("GLOBAL ENV");
    for (v, e) in &env {
        println!("{} := {}", v, e);
    }
    println!();

    resolve!(u64: var!(q), &env);
    resolve!(u64: var!(n), &env);

    // resolve!(Fn(): abs!{a.a}, &env);

    resolve!(Expression: var!(x), &env);
    resolve!(Expression: var!(x), &env);
    resolve!(Expression: var!(x), &env);
    resolve!(Expression: var!(n), &env);
    resolve!(Expression: var!(i), &env);
    resolve!(Expression: abs!{a.a}, &env);
    resolve!(Expression: abs!{a.n}, &env);
    resolve!(Expression: app!(a,x), &env);
    resolve!(Expression: app!(x,a), &env);
    resolve!(Expression: app!(x,x), &env);
    resolve!(Expression: app!(i,n), &env);
}
