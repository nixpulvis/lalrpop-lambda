#![feature(non_ascii_idents, box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;

use std::collections::HashSet;
use std::fmt;

#[macro_use]
mod macros;

/// A mutually recursive definition for all lambda expressions
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("λx.(x x)").is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variable(pub String);

/// An abstraction over a bound variable
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("λx.x").is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction(pub Variable, pub Box<Expression>);

/// An application of two expressions
///
/// ```
/// let parser = lalrpop_lambda::parse::ExpressionParser::new();
///
/// assert!(parser.parse("a b").is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application(pub Box<Expression>, pub Box<Expression>);

impl Expression {
    /// α-conversion
    pub fn rename(&self, old: &Variable, new: &Variable) -> Self {
        dbg!(old, new);
        unimplemented!()
    }

    /// β-reduction small-step semantics.
    ///
    /// η: Local completeness in natural deduction.
    ///
    /// Local reducibility in natural deduction.
    pub fn apply(&self, η: bool) -> Self {
        dbg!(η);
        unimplemented!()
    }

    /// Big-step natural semantics.
    ///
    /// η: Global completeness in natural deduction.
    ///
    /// Global reducibility in natural deduction.
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
                if let Expression::App(Application(box e1,
                                                   box Expression::Var(x)))
                   = body
                {
                    // λx.(e1 x) -> e1 whenever x does not appear free in e1.
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
                        // TODO: This should be the job of `apply` (aka →).
                        body.substitute(&e2, &id).normalize(η)
                    },
                    e @ _ => {
                        Expression::App(Application(box e.normalize(η), box e2.normalize(η)))
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


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Var(id) => {
                write!(f, "{}", id)
            },
            Expression::Abs(Abstraction(Variable(id), body)) => {
                write!(f, "(λ{}.{})", id, body)
            },
            Expression::App(Application(box e1, box e2)) => {
                write!(f, "({} {})", e1, e2)
            },
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


lalrpop_mod! {
    /// Parse lambda expression ASTs
    pub parse
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::ExpressionParser;

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
    }

    #[test]
    fn normalize_η() {
        assert_eq!(var!(f), abs!{x.app!(f,x)}.normalize(true));
        assert_eq!(abs!{x.app!(x,x)}, abs!{x.app!(x, x)}.normalize(true));
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
}
