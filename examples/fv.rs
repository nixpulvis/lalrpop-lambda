#![feature(box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

fn main() {
    dbg!(app!(abs!{x.app!(x,y)}, abs!{y.app!(x,y)}).free_variables());
    dbg!(app!(abs!{f.abs!{x.app!(f,x)}}, abs!{x.x}).free_variables());
}
