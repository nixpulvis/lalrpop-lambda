use crate::{Expression, Abstraction, Application, Variable};

/// A reduction strategy for an [`Expression`]
pub enum Strategy {
    // Innermost reductions

    // *ao* -> normal
    Applicative(bool),
    // *bv*: not inside abstractions -> weak normal
    CallByValue,
    // ha: ao + bv -> normal
    HybridApplicative,

    // Outermost reductions

    // *bn*: not inside abstractions -> weak head normal
    CallByName,
    // no: bn -> normal
    Normal(bool),
    // *he*: abstractions reduced only in head position -> head normal
    HeadSpine(bool),
    // hn: no + he -> normal
    HybridNormal,
}

impl Expression {
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
    /// use lalrpop_lambda::Strategy;
    /// use lalrpop_lambda::parse::ExpressionParser;
    ///
    /// let parser = ExpressionParser::new();
    /// let expression = parser.parse("((λx.(λy.x y) b) a)").unwrap();
    /// let normal = parser.parse("a b").unwrap();
    ///
    /// assert_eq!(normal, expression.normalize(&Strategy::Applicative(true)));
    /// assert_eq!(normal, expression.normalize(&Strategy::HeadSpine(false)));
    /// ```
    pub fn normalize(&self, strategy: &Strategy) -> Self {
        match *strategy {
            Strategy::CallByName     => self.bn(),
            Strategy::Normal(η)      => self.no(η),
            Strategy::CallByValue    => self.bv(),
            Strategy::Applicative(η) => self.ao(η),
            Strategy::HeadSpine(η)   => self.hs(η),
            _ => unimplemented!(),
        }
    }

    fn bn(&self) -> Self {
        match self {
            Expression::App(Application(box e1, box e2)) => {
                match e1.bn() {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2, &id).bn()
                    },
                    e @ _ => {
                        Expression::App(Application(box e, box e2.clone()))
                    }
                }
            },
            _ => self.clone(),
        }
    }

    fn no(&self, η: bool) -> Self {
        match self {
            Expression::Var(_) => self.clone(),
            Expression::Abs(Abstraction(id, box body)) => {
                // η-reduction
                if let Expression::App(Application(box e1,
                                                   box Expression::Var(x)))
                   = body
                {
                    if η && id == x && !e1.free_variables().contains(&id) {
                        return e1.no(η);
                    }
                }

                Expression::Abs(Abstraction(id.clone(),
                                            box body.no(η)))
            },
            Expression::App(Application(box e1, box e2)) => {
                match e1.bn() {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2, &id).no(η)
                    },
                    e @ _ => {
                        Expression::App(Application(box e.no(η), box e2.no(η)))
                    }
                }
            },
        }
    }

    fn bv(&self) -> Self {
        match self {
            Expression::App(Application(box e1, box e2)) => {
                match e1.bv() {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2.bv(), &id)
                    },
                    e @ _ => {
                        Expression::App(Application(box e, box e2.bv()))
                    }
                }
            },
            _ => self.clone(),
        }
    }

    fn ao(&self, η: bool) -> Self {
        match self {
            Expression::Var(_) => self.clone(),
            Expression::Abs(Abstraction(id, box body)) => {
                // η-reduction
                if let Expression::App(Application(box e1,
                                                   box Expression::Var(x)))
                   = body
                {
                    if η && id == x && !e1.free_variables().contains(&id) {
                        return e1.ao(η);
                    }
                }

                Expression::Abs(Abstraction(id.clone(),
                                            box body.ao(η)))
            },
            Expression::App(Application(box e1, box e2)) => {
                match e1.ao(η) {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2.ao(η), &id).ao(η)
                    },
                    e @ _ => {
                        Expression::App(Application(box e, box e2.ao(η)))
                    }
                }
            },
        }
    }

    fn hs(&self, η: bool) -> Self {
        match self {
            Expression::Abs(Abstraction(id, box body)) => {
                // η-reduction
                if let Expression::App(Application(box e1,
                                                   box Expression::Var(x)))
                   = body
                {
                    if η && id == x && !e1.free_variables().contains(&id) {
                        return e1.hs(η);
                    }
                }

                Expression::Abs(Abstraction(id.clone(),
                                            box body.hs(η)))
            },
            Expression::App(Application(box e1, box e2)) => {
                match e1.bn() {
                    Expression::Abs(Abstraction(id, body)) => {
                        body.substitute(&e2, &id)
                    },
                    e @ _ => {
                        Expression::App(Application(box e, box e2.clone()))
                    }
                }
            },
            _ => self.clone(),
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
                    let fresh = Variable(format!("{}'", id), None);
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
            Variable(new.0.clone(), new.1.clone())
        } else {
            self.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use crate::{abs,app,var,variable};
    use super::*;

    #[test]
    #[ignore]
    fn apply() {}

    #[test]
    fn normalize() {
        let strategy = Strategy::Applicative(false);

        assert_eq!(var!(a), app!(abs!{x.x}, a).normalize(&strategy));

        assert_eq!(app!(a, a), app!(abs!{x.app!(abs!{x.app!(x, x)}, a)}, b)
                                   .normalize(&strategy));
        assert_eq!(app!(a, b), app!(abs!{y.app!(a, y)}, b)
                                   .normalize(&strategy));
        assert_eq!(app!(b, a), app!(app!(abs!{x.abs!{y.app!(x, y)}}, b), a)
                                   .normalize(&strategy));
        assert_eq!(app!(b, b), app!(app!(abs!{x.abs!{y.app!(x, y)}}, b), b)
                                   .normalize(&strategy));

        assert_eq!(abs!{a.a}, app!(abs!{x.x}, abs!{a.a})
                                .normalize(&strategy));
        println!("{}", app!(abs!{f.abs!{x.app!(f,a)}}, abs!{x.x}));
        assert_eq!(abs!{x.a}, app!(abs!{f.abs!{x.app!(f,a)}}, abs!{x.x})
                                .normalize(&strategy));
    }

    #[test]
    fn normalize_capture_avoid() {
        let strategy = Strategy::Applicative(false);

        let expected = Expression::Abs(Abstraction(variable!("y"),
            box Expression::Abs(Abstraction(variable!("y'"),
                box Expression::Var(variable!("y"))))));
        let actual = abs!{y.app!(abs!{x.abs!{y.x}}, y)};
        assert_eq!(expected, actual.normalize(&strategy));

        let expected = abs!{f.abs!{x.app!(var!(f),
            Expression::Abs(Abstraction(variable!("x'"),
                                        box app!(app!(var!(f),
                                                      var!(x)),
                                                 var!("x'")))))}};
        let actual = app!(abs!{n.abs!{f.abs!{x.app!(f, app!(n, app!(f, x)))}}},
                          abs!{f.abs!{x.app!(f, x)}});
        assert_eq!(expected, actual.normalize(&strategy));

        let expected = Expression::Abs(Abstraction(variable!("x'"),
                                                   box var!(x)));
        let actual = app!(abs!{f.abs!{x.app!(f,a)}}, abs!{a.x})
                        .normalize(&strategy);
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
        assert_eq!(expected, actual.normalize(&strategy));
    }

    #[test]
    fn normalize_η() {
        let strategy = Strategy::Applicative(true);

        assert_eq!(var!(f), abs!{x.app!(f,x)}.normalize(&strategy));
        assert_eq!(abs!{x.app!(x,x)}, abs!{x.app!(x, x)}.normalize(&strategy));
        assert_eq!(abs!{f.f}, abs!{f.abs!{g.abs!{x.app!(app!(f,g),x)}}});
    }

    #[test]
    #[ignore]
    #[allow(non_snake_case)]
    fn normalize_Ω() {
        let strategy = Strategy::Applicative(false);

        let Ω = app!(abs!{x.app!(x,x)}, abs!{x.app!(x,x)});
        assert_eq!(abs!{x.x}, Ω.normalize(&strategy));
    }

    // TODO: Strategy testing.

    #[test]
    fn replace() {
        assert_eq!(var!(b), var!(a).replace(&variable!(a), &variable!(b)));
        assert_eq!(app!(b,b), app!(a,a).replace(&variable!(a), &variable!(b)));
        assert_eq!(abs!{b.b}, abs!{a.a}.replace(&variable!(a), &variable!(b)));
    }
}
