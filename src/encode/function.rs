use crate::Expression;

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


#[cfg(test)]
mod tests {
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
