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
//!     let one = app!({add1},{zero}).normalize(false);
//!     assert_eq!(Expression::from(1u64), one);
//!
//!     // Use a parsed identity function with other `Experssion`s.
//!     let id = parser.parse("λx.x").unwrap();
//!     let id_one = id(Expression::from(1u64)).normalize(false);
//!     assert_eq!(one, id_one);
//!
//!     // Use a parsed identity function with Rust `u64` numbers!
//!     // NOTE: This is a WIP.
//!     let id = parser.parse("λx.x").unwrap();
//!     let u64_id = <fn(u64) -> u64>::from(id.clone());
//!     assert_eq!(1, u64_id(1));
//! }
//! ```
#![feature(non_ascii_idents,
           box_syntax,
           box_patterns,
           fn_traits,
           unboxed_closures)]

#[macro_use]
extern crate lalrpop_util;

use std::collections::{HashSet, HashMap};
use std::fmt;

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
pub struct Variable(pub String);

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

    /// β-reduction small-step semantics (→)
    ///
    /// Represents local reducibility in natural deduction.
    ///
    /// - η: λx.(e1 x) -> e1 whenever x does not appear free in e1
    ///
    ///     Represents local completeness in natural deduction.
    pub fn apply(&self, η: bool) -> Self {
        dbg!(η);
        unimplemented!()
    }

    /// Big-step natural semantics (⇓)
    ///
    /// Represents global reducibility in natural deduction.
    ///
    /// TODO: Reduction strategy.
    ///
    /// - η: (see `Expression::apply`)
    ///
    /// ```
    /// let parser = lalrpop_lambda::parse::ExpressionParser::new();
    ///
    /// let expression = parser.parse("((λx.(λy.x y) b) a)").unwrap();
    /// let normal = parser.parse("a b").unwrap();
    ///
    /// assert_eq!(normal, expression.normalize(false));
    /// ```
    pub fn normalize(&self, η: bool) -> Self {
        match self {
            Expression::Var(_) => self.clone(),
            Expression::Abs(Abstraction(id, box body)) => {
                // η-reduction
                if let Expression::App(Application(box e1,
                                                   box Expression::Var(x)))
                   = body
                {
                    if η && id == x && !e1.free_variables().contains(&id) {
                        return e1.normalize(η);
                    }
                }

                Expression::Abs(Abstraction(id.clone(), box body.normalize(η)))
            },
            Expression::App(Application(box e1, box e2)) => {
                match e1.normalize(η) {
                    Expression::Abs(Abstraction(id, body)) => {
                        // (λx.t) s → t[x := s]
                        body.substitute(&e2, &id).normalize(η)
                    },
                    e @ _ => {
                        Expression::App(Application(box e.normalize(η),
                                                    box e2.normalize(η)))
                    }
                }
            },
        }
    }

    pub fn variables(&self) -> HashSet<Variable> {
        match self {
            Expression::Var(v) => set! { v.clone() },
            Expression::Abs(Abstraction(id, body)) => {
                body.variables().union(&set! { id.clone() }).cloned().collect()
            },
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
    /// free.insert(Variable("y".into()));
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
            Expression::Abs(Abstraction(id, body)) => {
                body.free_variables()
                    .difference(&set! { id.clone() })
                    .cloned()
                    .collect()
            },
            // FV(M N) = FV(M) ∪ FV(N).
            Expression::App(Application(e1, e2)) => {
                e1.free_variables()
                  .union(&e2.free_variables())
                  .cloned()
                  .collect()
            }
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
    pub fn resolve(&self, env: &HashMap<Variable,Expression>) -> Expression
    {
        match self {
            Expression::Var(id) => {
                if let Some(e) = env.get(id) {
                    e.clone()
                } else {
                    self.clone()
                }
            },
            Expression::Abs(Abstraction(id, box body)) => {
                // TODO: Check FV
                Expression::Abs(Abstraction(id.clone(),
                                            box body.resolve(env)))
            },
            Expression::App(Application(box e1, box e2)) => {
                app!({e1.resolve(env)}, {e2.resolve(env)})
            },
        }
    }

    /// self[x := v]
    fn substitute(&self, v: &Self, x: &Variable) -> Self {
        match self {
            Expression::Abs(Abstraction(id, box body)) => {
                if id == x || !v.free_variables().contains(id) {
                    Expression::Abs(Abstraction(id.clone(),
                                                box body.substitute(v, x)))
                } else {
                    let fresh = Variable(format!("{}'", id));
                    let body = body.replace(&id, &fresh);
                    Expression::Abs(Abstraction(fresh,
                                                box body.substitute(v, x)))
                }
            },
            Expression::Var(id) => {
                (if id == x { v } else { self }).clone()
            },
            Expression::App(Application(e1, e2)) => {
                Expression::App(Application(box e1.substitute(v, x),
                                            box e2.substitute(v, x)))
            }
        }
    }

    fn replace(&self, old: &Variable, new: &Variable) -> Self {
        match self {
            Expression::Var(v) => {
                Expression::Var(v.replace(old, new))
            },
            Expression::Abs(Abstraction(id, body)) => {
                Expression::Abs(Abstraction(id.replace(old, new),
                                            box body.replace(old, new)))
            },
            Expression::App(Application(e1, e2)) => {
                Expression::App(Application(box e1.replace(old, new),
                                            box e2.replace(old, new)))
            }
        }
    }
}

impl Variable {
    fn replace(&self, old: &Variable, new: &Variable) -> Self {
        if self.0 == old.0 {
            Variable(new.0.clone())
        } else {
            self.clone()
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Var(id) => {
                write!(f, "{:?}", id)
            },
            Expression::Abs(Abstraction(id, body)) => {
                write!(f, "(λ{:?}.{:?})", id, body)
            },
            Expression::App(Application(box e1, box e2)) => {
                write!(f, "({:?} {:?})", e1, e2)
            },
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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
    use pretty_assertions::assert_eq;
    use crate::parse::ExpressionParser;
    use super::*;

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
    fn apply() {}

    #[test]
    fn normalize() {
        assert_eq!(var!(a), app!(abs!{x.x}, a).normalize(false));

        assert_eq!(app!(a, a), app!(abs!{x.app!(abs!{x.app!(x, x)}, a)}, b)
                                   .normalize(false));
        assert_eq!(app!(a, b), app!(abs!{y.app!(a, y)}, b)
                                   .normalize(false));
        assert_eq!(app!(b, a), app!(app!(abs!{x.abs!{y.app!(x, y)}}, b), a)
                                   .normalize(false));
        assert_eq!(app!(b, b), app!(app!(abs!{x.abs!{y.app!(x, y)}}, b), b)
                                   .normalize(false));

        assert_eq!(abs!{a.a}, app!(abs!{x.x}, abs!{a.a})
                                .normalize(false));
        assert_eq!(abs!{x.a}, app!(abs!{f.abs!{x.app!(f,a)}}, abs!{x.x})
                                .normalize(false));
    }

    #[test]
    fn normalize_capture_avoid() {
        let expected = Expression::Abs(Abstraction(variable!("y"),
            box Expression::Abs(Abstraction(variable!("y'"),
                box Expression::Var(variable!("y"))))));
        let actual = abs!{y.app!(abs!{x.abs!{y.x}}, y)};
        assert_eq!(expected, actual.normalize(false));

        let expected = abs!{f.abs!{x.app!(var!(f),
            Expression::Abs(Abstraction(variable!("x'"),
                                        box app!(app!(var!(f),
                                                      var!(x)),
                                                 var!("x'")))))}};
        let actual = app!(abs!{n.abs!{f.abs!{x.app!(f, app!(n, app!(f, x)))}}},
                          abs!{f.abs!{x.app!(f, x)}});
        assert_eq!(expected, actual.normalize(false));

        let expected = Expression::Abs(Abstraction(variable!("x'"),
                                                   box var!(x)));
        let actual = app!(abs!{f.abs!{x.app!(f,a)}}, abs!{a.x})
                        .normalize(false);
        assert_eq!(expected, actual);

        let expected = abs!{f.abs!{x.app!(f,{
            let x2 = Expression::Abs(Abstraction(variable!("x''"),
                                                 box var!("x''")));
            let fx = app!(app!(f,x),{x2});
            Expression::Abs(Abstraction(variable!("x'"),
                                        box fx))
        })}};
        let actual = app!(abs!{n.abs!{f.abs!{x.app!(f,app!(n,app!(f,x)))}}},
                          app!(abs!{n.abs!{f.abs!{x.app!(f,app!(n,app!(f,x)))}}},
                               abs!{f.abs!{x.x}}));
        assert_eq!(expected, actual.normalize(false));
    }

    #[test]
    fn normalize_η() {
        assert_eq!(var!(f), abs!{x.app!(f,x)}.normalize(true));
        assert_eq!(abs!{x.app!(x,x)}, abs!{x.app!(x, x)}.normalize(true));
        assert_eq!(abs!{f.f}, abs!{f.abs!{g.abs!{x.app!(app!(f,g),x)}}});
    }

    #[test]
    #[ignore]
    #[allow(non_snake_case)]
    fn normalize_Ω() {
        let Ω = app!(abs!{x.app!(x,x)}, abs!{x.app!(x,x)});
        assert_eq!(abs!{x.x}, Ω.normalize(false));
    }

    #[test]
    fn replace() {
        assert_eq!(var!(b), var!(a).replace(&variable!(a), &variable!(b)));
        assert_eq!(app!(b,b), app!(a,a).replace(&variable!(a), &variable!(b)));
        assert_eq!(abs!{b.b}, abs!{a.a}.replace(&variable!(a), &variable!(b)));
    }

    #[test]
    #[ignore]
    fn variables() {}

    #[test]
    fn free_variables() {
        let parser = ExpressionParser::new();

        assert_eq!(set! { variable!(x) },
                   parser.parse(r"x").unwrap().free_variables());
        assert_eq!(set! { },
                   parser.parse(r"λx.x").unwrap().free_variables());
        assert_eq!(set! { variable!(f), variable!(x) },
                   parser.parse(r"f x").unwrap().free_variables());
        assert_eq!(set! { variable!(x), variable!(y) },
                   parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap().free_variables());
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
