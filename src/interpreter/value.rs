use crate::ast_nodes::Expr;

use super::GlobalScope;

#[derive(Debug, Clone)]
pub enum Value {
    Def { arg: String, value: Box<Value> },
    Id { name: String },
    BoundId { name: String, value: Box<Value> },
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
            Value::BoundId { name, value: _ } => name.to_string(),
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

    pub fn bind(&self, global: &GlobalScope) -> Value {
        match self {
            Value::Def { arg, value } => Value::Def {
                arg: arg.to_string(),
                value: value.bind(global).into(),
            },
            Value::Id { name } => match global.get(name) {
                Some(val) => Value::BoundId {
                    name: name.to_string(),
                    value: val.clone().into(),
                },
                None => self.clone(),
            },
            Value::BoundId { name: _, value: _ } => self.clone(),
            Value::Call { target, arg } => Value::Call {
                target: target.bind(global).into(),
                arg: arg.bind(global).into(),
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
