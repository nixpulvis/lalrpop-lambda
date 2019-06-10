use std::ops::{Not, BitAnd, BitOr, BitXor};
use crate::{Expression, Abstraction};

/// Church encoded booleans
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// use lalrpop_lambda::Expression;
///
/// # fn main() {
/// assert_eq!(λ!{a.λ!{b.a}}, Expression::from(true));
/// assert_eq!(λ!{a.λ!{b.b}}, Expression::from(false));
/// # }
/// ```
impl From<bool> for Expression {
    fn from(p: bool) -> Self {
        if p { λ!{a.λ!{b.a}} } else { λ!{a.λ!{b.b}} }
    }
}

/// Convert λ term back to native Rust type
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// assert_eq!(true , bool::from(λ!{a.λ!{b.a}}));
/// assert_eq!(false, bool::from(λ!{a.λ!{b.γ!(b,b)}}));
/// # }
/// ```
impl From<Expression> for bool {
    fn from(e: Expression) -> bool {
        if let Expression::Abs(Abstraction(a, box e1)) = e.normalize(true) {
           if let Expression::Abs(Abstraction(_, box e2)) = e1 {
               if let Expression::Var(p) = e2 {
                   return p == a
               }
            }
        }

        // Dirty hack, this should be an Option!
        false
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let t = λ!{a.λ!{b.a}};
///
/// assert_eq!(false, bool::from(!t.clone()));
/// assert_eq!(true, bool::from(!!t.clone()));
/// # }
/// ```
///
/// # Evaluation Strategy
///
/// Note that there are two version of not, depending on the evaluation
/// strategy that is chosen. We _currently_ only evaluate using application
/// order, so we choose the first option.
///
/// - Applicative evaluation order: λp.λa.λb.p b a
/// - Normal evaluation order: λp.p (λa.λb.b) (λa.λb.a)
impl Not for Expression {
    type Output = Self;

    fn not(self) -> Self {
        let not_app = λ!{p.λ!{a.λ!{b.γ!(γ!(p,b),a)}}};
        γ!({not_app},{self}).normalize(false)
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let t = λ!{a.λ!{b.a}};
/// let f = λ!{a.λ!{b.b}};
///
/// assert_eq!(true,  bool::from(t.clone() | t.clone()));
/// assert_eq!(true,  bool::from(t.clone() | f.clone()));
/// assert_eq!(true,  bool::from(f.clone() | t.clone()));
/// assert_eq!(false, bool::from(f.clone() | f.clone()));
/// # }
/// ```
impl BitOr for Expression {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let or = λ!{p.λ!{q.γ!(γ!(p,p),q)}};
        γ!(γ!({or},{self}),{other}).normalize(false)
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let t = λ!{a.λ!{b.a}};
/// let f = λ!{a.λ!{b.b}};
///
/// assert_eq!(true,  bool::from(t.clone() & t.clone()));
/// assert_eq!(false, bool::from(t.clone() & f.clone()));
/// assert_eq!(false, bool::from(f.clone() & t.clone()));
/// assert_eq!(false, bool::from(f.clone() & f.clone()));
/// # }
/// ```
impl BitAnd for Expression {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        let and = λ!{p.λ!{q.γ!(γ!(p,q),p)}};
        γ!(γ!({and},{self}),{other}).normalize(false)
    }
}

/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// let t = λ!{a.λ!{b.a}};
/// let f = λ!{a.λ!{b.b}};
///
/// assert_eq!(false, bool::from(t.clone() ^ t.clone()));
/// assert_eq!(true, bool::from(t.clone() ^ f.clone()));
/// assert_eq!(true, bool::from(f.clone() ^ t.clone()));
/// assert_eq!(false, bool::from(f.clone() ^ f.clone()));
/// # }
/// ```
impl BitXor for Expression {
    type Output = Self;

    // TODO: This is proving we need α-renaming.
    fn bitxor(self, other: Self) -> Self {
        let not_app = λ!{p.λ!{a.λ!{b.γ!(γ!(p,b),a)}}};
        let xor = λ!{p.λ!{q.γ!(γ!(p,γ!({not_app},q)),q)}};
        γ!(γ!({xor},{self}),{other}).normalize(false)
    }
}



#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn true_() {
        assert_eq!(true, Expression::from(true).into());
    }

    #[test]
    fn false_() {
        assert_eq!(false, Expression::from(false).into());
    }

    #[test]
    fn not() {
        assert_eq!(false, (!Expression::from(true)).into());
    }

    #[test]
    fn or() {
        assert_eq!(Expression::from(true), Expression::from(true) |
                                           Expression::from(true));
        assert_eq!(Expression::from(true), Expression::from(false) |
                                           Expression::from(true));
        assert_eq!(Expression::from(true), Expression::from(true) |
                                           Expression::from(false));
        assert_eq!(Expression::from(false), Expression::from(false) |
                                            Expression::from(false));
    }

    #[test]
    fn and() {
        assert_eq!(Expression::from(true), Expression::from(true) &
                                           Expression::from(true));
        assert_eq!(Expression::from(false), Expression::from(false) &
                                            Expression::from(true));
        assert_eq!(Expression::from(false), Expression::from(true) &
                                            Expression::from(false));
        assert_eq!(Expression::from(false), Expression::from(false) &
                                            Expression::from(false));
    }

    #[test]
    fn xor() {
        assert_eq!(Expression::from(false), Expression::from(true) ^
                                            Expression::from(true));
        assert_eq!(Expression::from(true), Expression::from(false) ^
                                           Expression::from(true));
        assert_eq!(Expression::from(true), Expression::from(true) ^
                                           Expression::from(false));
        assert_eq!(Expression::from(false), Expression::from(false) ^
                                            Expression::from(false));
    }
}
