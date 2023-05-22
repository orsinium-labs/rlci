use crate::ast_nodes::Expr;
use std::rc::Rc;

pub enum Value {
    Def { arg: String, value: Rc<Value> },
    Id { name: String },
    BoundId { name: String, value: Rc<Value> },
    Call { target: Rc<Value>, arg: Rc<Value> },
}

impl Value {
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Def { arg, expr } => Value::Def {
                arg: arg.to_string(),
                value: Rc::new(Value::from_expr(expr)),
            },
            Expr::Call { target, arg } => Value::Call {
                target: Rc::new(Value::from_expr(target)),
                arg: Rc::new(Value::from_expr(arg)),
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
