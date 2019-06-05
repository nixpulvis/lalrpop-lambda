#![feature(non_ascii_idents, box_syntax, box_patterns)]

#[macro_use]
extern crate lalrpop_util;

use std::collections::HashSet;
use std::fmt;

macro_rules! map(
    { $($value:expr),* } => {
        {
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($value);
            )*
            m
        }
     };
);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Var(Variable),
    Abs(Abstraction),
    App(Application),
}

impl Expression {
    pub fn normalize(&self) -> Self {
        match self {
            Expression::Var(_) |
            Expression::Abs(_) => self.clone(),
            Expression::App(Application(box e1, box e2)) => {
                match e1.normalize() {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2, &id).normalize()
                    },
                    e @ _ => {
                        Expression::App(Application(box e, box e2.clone()))
                    }
                }
            }
        }
    }

    pub fn free_variables(&self) -> HashSet<Variable> {
        match self {
            Expression::Var(v) => map! { v.clone() },
            Expression::Abs(Abstraction(id, body)) => {
                body.free_variables().difference(&map! { id.clone() }).cloned().collect()
            },
            Expression::App(Application(e1, e2)) => {
                e1.free_variables().union(&e2.free_variables()).cloned().collect()
            }
        }
    }

    pub fn replace(&self, old: &Variable, new: &Variable) -> Self {
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

    pub fn substitute(&self, value: &Self, variable: &Variable) -> Self {
        match self {
            Expression::Abs(Abstraction(id, body)) => {
                let free = dbg!(value.free_variables());
                if id == variable {
                    self.clone()
                } else if !free.contains(id) {
                    Expression::Abs(Abstraction(variable.clone(),
                                                box body.substitute(value, variable)))
                } else {
                    let fresh = Variable("??".to_string());
                    let new_body = body.replace(&variable, &fresh);
                    Expression::Abs(Abstraction(fresh,
                                                box new_body.substitute(value, variable)))
                }
            },
            Expression::Var(id) => {
                if id == variable {
                    value.clone()
                } else {
                    self.clone()
                }
            },
            Expression::App(Application(e1, e2)) => {
                Expression::App(Application(box e1.substitute(value, variable),
                                            box e2.substitute(value, variable)))
            }
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variable(pub String);

impl Variable {
    fn replace(&self, old: &Variable, new: &Variable) -> Self {
        if self.0 == old.0 {
            Variable(new.0.clone())
        } else {
            self.clone()
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction(pub Variable, pub Box<Expression>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application(pub Box<Expression>, pub Box<Expression>);

lalrpop_mod!(pub lambda);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lambda::ExpressionParser;

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
    fn replace() {
        let parser = ExpressionParser::new();
        let a_var = Variable("a".into());
        let b_var = Variable("b".into());

        let a = parser.parse(r"a").unwrap();
        let b = parser.parse(r"b").unwrap();
        assert_eq!(b, a.replace(&a_var, &b_var));

        let id = parser.parse(r"λa.a").unwrap();
        let id2 = parser.parse(r"λb.b").unwrap();
        assert_eq!(id2, id.replace(&a_var, &b_var));
    }

    // #[test]
    // fn substitute() {
    //     let parser = ExpressionParser::new();

    //     let a = parser.parse(r"a").unwrap();
    //     let id = parser.parse(r"λx.x").unwrap();
    //     assert_eq!(a, id.substitute(&a));
    //     let ω = parser.parse(r"λx.(x x)").unwrap();
    //     let ids = parser.parse(r"((λx.x) (λx.x))").unwrap();
    //     assert_eq!(ids, ω.substitute(&id));
    // }

    #[test]
    fn normalize() {
        let parser = ExpressionParser::new();

        // let odd = parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap();
        // let normal = odd.normalize();

        let expected = parser.parse(r"\x.x").unwrap();
        let actual = parser.parse(r"(\f.\x.(f x)) (\x.x)").unwrap();
        println!("{} : {} -> {}", expected, actual, actual.normalize());
        assert_eq!(expected, actual.normalize());
        // assert_eq!(parser.parse(r"\x.x").unwrap(),
        //            parser.parse(r"(\x. (x x)) (\x. (x x))").unwrap().normalize());
    }

    #[test]
    fn free_variables() {
        let parser = ExpressionParser::new();

        assert_eq!(map! { Variable("x".into()) },
                   parser.parse(r"x").unwrap().free_variables());
        assert_eq!(map! { },
                   parser.parse(r"λx.x").unwrap().free_variables());
        assert_eq!(map! { Variable("x".into()), Variable("y".into()) },
                   parser.parse(r"(λx.(x y)) (λy.(x y))").unwrap().free_variables());
    }
}
