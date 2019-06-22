#![feature(non_ascii_idents, box_syntax)]
extern crate lalrpop_lambda;

use lalrpop_lambda::Expression;

fn main() {
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

    let t = true;
    let lt = Expression::from(t);
    let tt = bool::from(lt.clone());
    println!("{} -> {} -> {}", t, lt, tt);

    let f = false;
    let lf = Expression::from(f);
    let ff = bool::from(lf.clone());
    println!("{} -> {} -> {}", f, lf, ff);
}
