//! This project started as just the parse.lalrpop and AST, but grew into a bit
//! more.
//!
//! Evaluation of λ-expressions is _currently_ done in a single big-step
//! semantics [`Expression::normalize`] function. The reduction strategy is
//! so far only configurable by η.
//!
//! See the `impl From` items under [`Expression`]. These define conversions
//! between Rust and λ-expressions. These are all defined in `mod encode`.
//!
//! ```
//! #![feature(box_syntax)]
//!
//! #[macro_use]
//! extern crate lalrpop_lambda;
//!
//! use lalrpop_lambda::Strategy;
//!
//! fn main() {
//!     use lalrpop_lambda::Expression;
//!     use lalrpop_lambda::parse::ExpressionParser;
//!
//!     // Define an expression parser, for shortest lambda term strings.
//!     let parser = ExpressionParser::new();
//!
//!     // The successor Church numeral function.
//!     let add1 = parser.parse("λn.λf.λx.f (n f x)").unwrap();
//!
//!     // The first two church numerals.
//!     let zero = Expression::from(0u64);
//!     let one = app!({add1},{zero})
//!         .normalize(&Strategy::Applicative(false));
//!     assert_eq!(Expression::from(1u64), one);
//!
//!     // Use a parsed identity function with other `Experssion`s.
//!     let id = parser.parse("λx.x").unwrap();
//!     let id_one = id(Expression::from(1u64))
//!         .normalize(&Strategy::Applicative(false));
//!     assert_eq!(one, id_one);
//! }
//! ```
#![feature(non_ascii_idents, box_patterns, fn_traits, unboxed_closures)]

#[macro_use]
extern crate lalrpop_util;

#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

#[cfg(test)]
extern crate pretty_assertions;

use std::collections::{HashMap, HashSet};
use std::fmt;

#[cfg(feature = "wasm")]
pub mod wasm;

// TODO: Polish and test.
mod normal;
pub use self::normal::Strategy;

// The wonderful and easy to use `λ` and `abs!` macros.
//
// As well as an implementation of `set!` and `map!` taken from:
// [bluss/maplit](https://github.com/bluss/maplit).
#[macro_use]
mod macros;

// Church encoded λ-calculus data types, and conversions to Rust data types
mod encode;

/// A mutually recursive definition for all lambda expressions
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("λx.(x x)").is_ok());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    Var(Variable),
    Abs(Abstraction),
    App(Application),
}

/// A potentially free variable
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("x").is_ok());
/// ```
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Variable(pub String, pub Option<String>);

/// An abstraction over a bound variable
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("λx.x").is_ok());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Abstraction(pub Variable, pub Box<Expression>);

/// An application of two expressions
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("a b").is_ok());
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Application(pub Box<Expression>, pub Box<Expression>);

impl Expression {
    /// α-conversion
    pub fn rename(&self, old: &Variable, new: &Variable) -> Self {
        dbg!(old, new);
        unimplemented!()
    }

    pub fn variables(&self) -> HashSet<Variable> {
        match self {
            Expression::Var(v) => set! { v.clone() },
            Expression::Abs(Abstraction(id, body)) => body
                .variables()
                .union(&set! { id.clone() })
                .cloned()
                .collect(),
            Expression::App(Application(e1, e2)) => {
                e1.variables().union(&e2.variables()).cloned().collect()
            }
        }
    }

    /// FV(M) is the set of variables in M, not closed by a λ term.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use lalrpop_lambda::Variable;
    ///
    /// let parser = lalrpop_lambda::parse::ExpressionParser::new();
    ///
    /// let mut free = HashSet::new();
    /// free.insert(Variable("y".into(), None));
    ///
    /// let expression = parser.parse("λx.(x y)").unwrap();
    ///
    /// assert_eq!(free, expression.free_variables());
    /// ```
    pub fn free_variables(&self) -> HashSet<Variable> {
        match self {
            // FV(x) = { x }, where x is a variable.
            Expression::Var(id) => set! { id.clone() },
            // FV(λx.M) = FV(M) \ { x }.
            Expression::Abs(Abstraction(id, body)) => body
                .free_variables()
                .difference(&set! { id.clone() })
                .cloned()
                .collect(),
            // FV(M N) = FV(M) ∪ FV(N).
            Expression::App(Application(e1, e2)) => e1
                .free_variables()
                .union(&e2.free_variables())
                .cloned()
                .collect(),
        }
    }

    /// ```
    /// # #![feature(box_syntax)]
    /// # #[macro_use]
    /// # extern crate lalrpop_lambda;
    /// use std::collections::HashMap;
    ///
    /// # fn main() {
    /// let mut env = HashMap::new();
    /// env.insert(variable!(id), abs!{x.x});
    /// env.insert(variable!(ad), abs!{x.y});
    /// env.insert(variable!(x), 1.into());
    ///
    /// assert_eq!(var!(q), var!(q).resolve(&env));
    /// assert_eq!(1u64, var!(x).resolve(&env).into());
    ///
    /// // Works with functions too!
    /// let id: fn(u64) -> u64 = var!(id).resolve(&env).into();
    /// assert_eq!(1, id(1));
    /// let ad: fn(u64) -> u64 = var!(ad).resolve(&env).into();
    /// assert_eq!(u64::from(var!(y)), ad(0));
    /// assert_eq!(u64::from(var!(y)), ad(1));
    /// # }
    /// ```
    pub fn resolve(&self, env: &HashMap<Variable, Expression>) -> Expression {
        match self {
            Expression::Var(id) => {
                if let Some(e) = env.get(id) {
                    e.clone()
                } else {
                    self.clone()
                }
            }
            Expression::Abs(Abstraction(id, box body)) => {
                // TODO: Check FV
                Expression::Abs(Abstraction(id.clone(), Box::new(body.resolve(env))))
            }
            Expression::App(Application(box e1, box e2)) => {
                app!({ e1.resolve(env) }, { e2.resolve(env) })
            }
        }
    }
}

impl Expression {
    pub fn build_abs(lambs: usize, ids: Vec<Variable>, body: Option<Expression>) -> Self {
        // TODO: Make the body an Option too.
        let mut abs = body.unwrap_or(var!(""));

        let id_count = ids.len();
        // Curry multi args.
        for i in ids.into_iter().rev() {
            abs = Expression::Abs(Abstraction(i, Box::new(abs)));
        }

        // Wrap in as many extra lambdas as requested.
        for l in 0..lambs {
            // Skip the first lambda if given any ids, since the id will have
            // already generated above.
            if l == 0 && id_count > 0 {
                continue;
            }
            abs = Expression::Abs(Abstraction(variable!(""), Box::new(abs)));
        }

        abs
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Var(id) => {
                write!(f, "{:?}", id)
            }
            Expression::Abs(Abstraction(id, body)) => {
                write!(f, "(λ{:?}.{:?})", id, body)
            }
            Expression::App(Application(box e1, box e2)) => {
                write!(f, "({:?} {:?})", e1, e2)
            }
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ty) = &self.1 {
            write!(f, "{}:{}", self.0, ty)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

lalrpop_mod! {
    /// Parse λ-expressions
    pub parse
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::ExpressionParser;
    use pretty_assertions::assert_eq;

    #[test]
    fn variable() {
        assert!(ExpressionParser::new().parse(r"x").is_ok());
    }

    #[test]
    fn abstraction() {
        assert!(ExpressionParser::new().parse(r"\x.x").is_ok());
        assert!(ExpressionParser::new().parse(r"\x. x").is_ok());
        assert!(ExpressionParser::new().parse(r"\x.(x)").is_ok());
        assert!(ExpressionParser::new().parse(r"\x. (x)").is_ok());
    }

    #[test]
    fn application() {
        assert!(ExpressionParser::new().parse(r"x x").is_ok());
        assert!(ExpressionParser::new().parse(r"(x y)").is_ok());
        assert!(ExpressionParser::new().parse(r"(\x.x y)").is_ok());
    }

    #[test]
    #[ignore]
    fn rename() {}

    #[test]
    #[ignore]
    fn variables() {}

    #[test]
    fn free_variables() {
        let parser = ExpressionParser::new();

        assert_eq!(
            set! { variable!(x) },
            parser.parse(r"x").unwrap().free_variables()
        );
        assert_eq!(set! {}, parser.parse(r"λx.x").unwrap().free_variables());
        assert_eq!(
            set! { variable!(f), variable!(x) },
            parser.parse(r"f x").unwrap().free_variables()
        );
        assert_eq!(
            set! { variable!(x), variable!(y) },
            parser
                .parse(r"(λx.(x y)) (λy.(x y))")
                .unwrap()
                .free_variables()
        );
    }

    #[test]
    fn resolve() {
        let env = map! {
            variable!(n) => 1.into(),
        };

        assert_eq!(var!(q), var!(q).resolve(&env));
        assert_eq!(1u64, var!(n).resolve(&env).into());

        // TODO: Add more, starting with examples/env.rs.
    }
}
