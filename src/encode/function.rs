use crate::{Expression, Abstraction};

/// Function call support for an `Expression`.
///
/// ```
/// # #![feature(box_syntax)]
/// # #[macro_use]
/// # extern crate lalrpop_lambda;
/// # fn main() {
/// assert_eq!(0u64, λ!{x.x}(0).into());
/// assert_eq!(γ!(γ!(a,b),0), γ!(a,b)(0));
/// # }
/// ```
impl<T> FnOnce<(T,)> for Expression
    where T: Into<Expression> +
             From<Expression>
{
    // TODO: Return Option<T> here too.
    type Output = Expression;

    extern "rust-call" fn call_once(self, t: (T,)) -> Expression {
        γ!({self},t.0.into())
    }
}

impl From<Expression> for fn(u64) -> u64 {
    fn from(e: Expression) -> Self {
        match e {
            Expression::Abs(Abstraction(ref lid, box ref e1)) => {
                match e1 {
                    Expression::Var(ref rid) if lid == rid => {
                        |x| x
                    },
                    Expression::Var(_) => {
                        |_| 0
                    },
                    _ => unreachable!(),
                }
            },
            _ => |_| panic!("not a function"),
        }
    }
}

impl From<fn(u64) -> u64> for Expression {
    fn from(_f: fn(u64) -> u64) -> Self {
        // TODO: Since we can't call `f` now, we should bind it to an `env` and
        // add a reference to f here.
        //
        // un-η
        abs!{x.app!(f,x)}
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn var() {
        let one = abs!{f.abs!{x.app!(f,x)}};
        assert_eq!(app!(x,{one}), var!(x)(1));
    }

    #[test]
    fn abs() {
        assert_eq!(5u64, abs!{x.x}(5).into());
    }

    #[test]
    fn app() {
        let zero = abs!{f.abs!{x.x}};
        assert_eq!(app!(app!(a,b),{zero}), app!(a,b)(0));
    }
}
