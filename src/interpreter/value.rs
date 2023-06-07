use super::GlobalScope;
use crate::ast_nodes::Expr;
use anyhow::Context;

#[derive(Debug, Clone)]
pub enum Value {
    Def {
        arg: String,
        value: Box<Value>,
    },
    Id {
        name: String,
    },
    BoundId {
        name: String,
        value: Box<Value>,
        global: bool,
    },
    Call {
        target: Box<Value>,
        arg: Box<Value>,
    },
}

impl Value {
    /// Convert an AST node into a Value.
    ///
    /// `Expr` and `Value` are very similar but not the same.
    /// Their semantic is slightly different and then `Value` may contain
    /// runtime-only infromation. In our case, IDs can be bound,
    /// so `Value` additionally has `BoundId`.
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Def { arg, expr } => Value::Def {
                arg: arg.to_string(),
                value: Value::from_expr(expr).into(),
            },
            Expr::Call { target, arg } => Value::Call {
                target: Value::from_expr(target).into(),
                arg: Value::from_expr(arg).into(),
            },
            Expr::Id { name } => Value::Id {
                name: name.to_string(),
            },
        }
    }

    /// Represent the value as a valid human-readable expression.
    ///
    /// The name and the idea comes from Python's `__repr__` magic method.
    /// The main idea is that you can copy-paste the result of this function into REPL
    /// and (assuming all globals are the same) get the same evaluation result.
    ///
    /// The function is primarily used by the REPL to nicely format execution result.
    pub fn repr(&self) -> String {
        // This is a convenient way to more briefly referrer to the items
        // of the current enum. So, instead of `Value::Def` we can write just `Def`.
        use Value::*;
        match self {
            Def { arg, value } => {
                format!("λ{arg} {}", value.repr())
            }
            Id { name } => name.to_string(),
            // Global IDs are better to be referred in repr by their name.
            BoundId {
                name, global: true, ..
            } => name.to_string(),
            // Local bound IDs should be represented by their values.
            // If you repr them by name, the result of `(λa λb a) true` will be
            // represented as `λb a` and it won't be clear what is `a` in this case.
            // However, if you repr them by value, the repr of the result
            // will be `λb true`.
            BoundId { value, .. } => value.repr(),
            Call { target, arg } => {
                let mut tr = target.repr();
                let mut ar = arg.repr();
                if tr.contains('λ') {
                    tr = format!("({tr})");
                }
                if ar.contains(' ') {
                    ar = format!("({ar})");
                }
                format!("{tr} {ar}")
            }
        }
    }

    pub fn bind_global(&self, global: &GlobalScope) -> Value {
        use Value::*;
        match self {
            Def { arg, value } => Def {
                arg: arg.to_string(),
                value: value.bind_global(global).into(),
            },
            Id { name } => match global.get(name) {
                Some(val) => BoundId {
                    name: name.to_string(),
                    value: val.clone().into(),
                    global: true,
                },
                None => self.clone(),
            },
            BoundId { .. } => self.clone(),
            Call { target, arg } => Call {
                target: target.bind_global(global).into(),
                arg: arg.bind_global(global).into(),
            },
        }
    }

    pub fn eval(&self) -> anyhow::Result<Value> {
        use Value::*;
        Ok(match self {
            Def { arg: _, value: _ } => self.clone(),
            Id { name } => anyhow::bail!("unbound variable `{name}`"),
            BoundId { name, value, .. } => {
                value.eval().context(format!("failure executing {name}"))?
            }
            Call { target, arg } => {
                // This is the star of the show. The main point of the evaluation
                // is to call all functions.
                target.call(arg).context("failure calling a function")?
            }
        })
    }

    pub fn call(&self, arg_value: &Value) -> anyhow::Result<Value> {
        use Value::*;
        match self {
            Def {
                arg: arg_name,
                value: expr,
            } => {
                let expr = expr.bind_local(arg_name, arg_value);
                expr.eval()
            }
            Id { name } => anyhow::bail!("unbound variable `{name}`"),
            BoundId { value, .. } => value.call(arg_value),
            Call { target, arg } => {
                let value = target.call(arg)?;
                value.call(arg_value)
            }
        }
    }

    fn bind_local(&self, lname: &str, lvalue: &Value) -> Value {
        use Value::*;
        match self {
            Def { arg, value } => {
                // Do not bind the local variable to functions that will shadow it anyway.
                // This is not just a performance improvement. If we bind a variable
                // that is meant to be rebind later, we may get a wrong repr.
                // For example: `(λa λa a) id`.
                // The repr for the evaluation result of this expression should be `λa a`
                // but without the check below it will be `λa id` which is wrong.
                if arg == lname {
                    self.clone()
                } else {
                    Def {
                        arg: arg.to_string(),
                        value: value.bind_local(lname, lvalue).into(),
                    }
                }
            }
            Id { name } | BoundId { name, .. } => {
                if name == lname {
                    BoundId {
                        name: name.to_string(),
                        value: lvalue.clone().into(),
                        global: false,
                    }
                } else {
                    self.clone()
                }
            }
            Call { target, arg } => {
                let target = target.bind_local(lname, lvalue);
                let arg = arg.bind_local(lname, lvalue);
                Call {
                    target: target.into(),
                    arg: arg.into(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_nodes::Stmt;
    use crate::parse;
    use rstest::rstest;

    #[rstest]
    #[case::id(r#"id"#, "id")]
    #[case::id(r#"\x x"#, "λx x")]
    #[case::id(r#"\a \b a b"#, "λa λb a b")]
    #[case::id(r#"\a \b (\c c) b"#, "λa λb (λc c) b")]
    #[case::id(r#"\a \b a (b a)"#, "λa λb a (b a)")]
    #[case::id(r#"\a \b a b a"#, "λa λb a b a")]
    #[case::id(r#"\a \b (a b) a"#, "λa λb a b a")]
    fn parse_and_repr(#[case] input: &str, #[case] exp: &str) {
        let module = parse(input).unwrap();
        assert_eq!(module.stmts.len(), 1);
        let stmt = &module.stmts[0];
        match stmt {
            Stmt::Expr { expr } => assert_eq!(Value::from_expr(expr).repr(), exp),
            _ => panic!("bad statement"),
        }
    }
}
