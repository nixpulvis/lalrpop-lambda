use std::ops::{Add, Mul};
use crate::{Expression, Abstraction, Application, Variable};

/// Church encoded natural numbers
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// use lalrpop_lambda::Expression;
///
/// # fn main() {
/// assert_eq!(λ!{f.λ!{x.x}}, Expression::from(0));
/// assert_eq!(λ!{f.λ!{x.γ!(f,x)}}, Expression::from(1));
/// assert_eq!(λ!{f.λ!{x.γ!(f,γ!(f,γ!(f,x)))}}, Expression::from(3));
/// # }
/// ```
impl From<u64> for Expression {
    fn from(n: u64) -> Self {
        let succ = λ!{n.λ!{f.λ!{x.γ!(f, γ!(γ!(n, f), x))}}};
        let mut e = λ!{f.λ!{x.x}};
        for _ in 0..n {
            e = app!({&succ}, {&e}).normalize(false);
        }
        e
    }
}

/// Convert λ term back to native Rust type
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// assert_eq!(0, u64::from(λ!{f.λ!{x.x}}));
/// assert_eq!(1, u64::from(λ!{f.λ!{x.γ!(f,x)}}));
/// assert_eq!(3, u64::from(λ!{f.λ!{x.γ!(f,γ!(f,γ!(f,x)))}}));
/// # }
/// ```
impl From<Expression> for u64 {
    fn from(e: Expression) -> u64 {
        // TODO: It would be ideal to use the Fn conversion and a way to "bind" `f` to u64::add.
        //
        // XXX: In fact more than ideal, this really should only be able to return an `Option<u64>`
        // since there are lambda terms which can evaluate to something which is not a chruch
        // encoded function.
        match e.normalize(true) {
            Expression::Var(id) => {
                if id == variable!(f) { 1 } else { 0 }
            },
            Expression::Abs(Abstraction(Variable(_id), box body)) => {
                u64::from(body)
            },
            Expression::App(Application(box e1, box e2)) => {
                u64::from(e1) + u64::from(e2)
            },
        }
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let one = λ!{f.λ!{x.γ!(f,x)}};
/// let two = one.clone() + one.clone();
/// assert_eq!(2, u64::from(two.clone()));
/// assert_eq!(4, u64::from(two.clone() + two.clone()));
/// # }
/// ```
impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let add = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,f),γ!(γ!(n,f),x))}}}};
        γ!(γ!({add},{self}),{other}).normalize(false)
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let one = λ!{f.λ!{x.γ!(f,x)}};
/// let two = one.clone() + one.clone();
/// assert_eq!(1, u64::from(one.clone() * one.clone()));
/// assert_eq!(4, u64::from(two.clone() * two.clone()));
/// # }
/// ```
impl Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mul = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,γ!(n,f)),x)}}}};
        γ!(γ!({mul},{self}),{other}).normalize(false)
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::parse::ExpressionParser;
    use super::*;

    // // TODO: Move these to tests as we finalize.
    // dbg!(u64::from(var!(x)));
    // dbg!(u64::from(var!(f)));
    // dbg!(u64::from(app!(f,app!(f,x))));
    // dbg!(u64::from(abs!{f.app!(f,app!(f,x))}));
    // dbg!(u64::from(abs!{f.abs!{x.app!(f,app!(f,x))}}));

    #[test]
    fn u64() {
        assert_eq!(0u64, Expression::from(0).into());
        assert_eq!(5u64, Expression::from(5).into());
    }

    #[test]
    fn zero() {
        // TODO: Should this be correct? What to do about smaller terms?
        assert_eq!(0, u64::from(λ!{x.x}));
    }


    #[test]
    fn one() {
        let ω = ExpressionParser::new().parse("λx.x x").unwrap();

        // TODO: Should this be correct? What to do about smaller terms?
        assert_eq!(1, u64::from(ω(Expression::from(1))));
    }

    #[test]
    fn add() {
        assert_eq!(Expression::from(5), Expression::from(2) +
                                        Expression::from(3));
    }

    #[test]
    fn multiply() {
        assert_eq!(Expression::from(6), Expression::from(2) * Expression::from(3));
    }

    // app!(n, (\f.\x.(f (f x)))) -> 2^n
}
