use std::ops::{Add, Mul};
use crate::{Expression, Abstraction, Application, Variable};

/// Church encoded natural numbers.

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

impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let add = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,f),γ!(γ!(n,f),x))}}}};
        γ!(γ!({add},{self}),{other}).normalize(false)
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mul = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,γ!(n,f)),x)}}}};
        γ!(γ!({mul},{self}),{other}).normalize(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64() {
        assert_eq!(0u64, Expression::from(0).into());
        assert_eq!(5u64, Expression::from(5).into());
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
}
