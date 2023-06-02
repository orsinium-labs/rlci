use super::{GlobalScope, LocalScope};
use crate::ast_nodes::Expr;
use anyhow::Context;

#[derive(Debug, Clone)]
pub enum Value {
    Def { arg: String, value: Box<Value> },
    Id { name: String },
    GlobalId { name: String, value: Box<Value> },
    LocalId { name: String, value: Box<Value> },
    Call { target: Box<Value>, arg: Box<Value> },
}

impl Value {
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

    pub fn repr(&self) -> String {
        match self {
            Value::Def { arg, value } => {
                format!("λ{arg} {}", value.repr())
            }
            Value::Id { name } => name.to_string(),
            Value::GlobalId { name, value: _ } => name.to_string(),
            Value::LocalId { name: _, value } => value.repr(),
            Value::Call { target, arg } => {
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
                Some(val) => GlobalId {
                    name: name.to_string(),
                    value: val.clone().into(),
                },
                None => self.clone(),
            },
            GlobalId { name: _, value: _ } | LocalId { name: _, value: _ } => self.clone(),
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
            GlobalId { name, value } | LocalId { name, value } => {
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
                let local = LocalScope::new(arg_name.to_string(), arg_value);
                let expr = expr.bind_local(&local);
                expr.eval()
            }
            Id { name } => anyhow::bail!("unbound variable `{name}`"),
            LocalId { name: _, value } | GlobalId { name: _, value } => value.call(arg_value),
            Call { target, arg } => {
                let value = target.call(arg)?;
                value.call(arg_value)
            }
        }
    }

    fn bind_local(&self, local: &LocalScope) -> Value {
        use Value::*;
        match self {
            Def { arg, value } => Def {
                arg: arg.to_string(),
                value: value.bind_local(local).into(),
            },
            Id { name } | LocalId { name, value: _ } | GlobalId { name, value: _ } => {
                match local.get(name) {
                    Some(val) => LocalId {
                        name: name.to_string(),
                        value: val.clone().into(),
                    },
                    None => self.clone(),
                }
            }
            Call { target, arg } => Call {
                target: target.bind_local(local).into(),
                arg: arg.bind_local(local).into(),
            },
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
