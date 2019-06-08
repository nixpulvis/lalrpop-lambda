use crate::{Expression, Abstraction};

impl<T> FnOnce<(T,)> for Expression
    where T: Into<Expression> +
             From<Expression>
{
    // TODO: Return Option<T> here too.
    type Output = T;

    extern "rust-call" fn call_once(self, _t: (T,)) -> T {
        match self {
            Expression::Abs(Abstraction(_id, box body)) => {
                // TODO: resolve self with env { id => t.into() }.
                body.into()
            },
            e @ _ => {
                // TODO: resolve self.
                e.into()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn call() {
        assert_eq!(1, abs!{x.x}(1));
    }
}
