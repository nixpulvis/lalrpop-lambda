use std::ops;
use crate::Expression;

/// Church encoded natural numbers.
pub struct Natural(Expression);

impl Natural {
    pub fn expression(&self) -> &Expression {
        &self.0
    }
}

impl From<u64> for Natural {
    fn from(n: u64) -> Self {
        let succ = λ!{n.λ!{f.λ!{x.γ!(f, γ!(γ!(n, f), x))}}};
        let mut e = λ!{f.λ!{x.x}};
        for _ in 0..n {
            e = app!({&succ}, {&e}).normalize(false);
        }
        Natural(e)
    }
}

impl Into<u64> for Natural {
    fn into(self) -> u64 {
        unimplemented!();
    }

}

impl ops::Add for Natural {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let add = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,f),γ!(γ!(n,f),x))}}}};
        Natural(γ!(γ!({add},{self.0}),{other.0}).normalize(false))
    }
}

impl ops::Mul for Natural {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mul = λ!{m.λ!{n.λ!{f.λ!{x.γ!(γ!(m,γ!(n,f)),x)}}}};
        Natural(γ!(γ!({mul},{self.0}),{other.0}).normalize(false))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Natural::from(5).expression(),
                   (Natural::from(2) + Natural::from(3)).expression());
    }

    #[test]
    fn multiply() {
        assert_eq!(Natural::from(6).expression(),
                   (Natural::from(2) * Natural::from(3)).expression());
    }
}
