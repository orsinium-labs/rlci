use anyhow::Context;

use crate::ast_nodes::{Expr, Module, Stmt};
use crate::interpreter::*;

pub struct Session {
    global: GlobalScope,
}

impl Session {
    pub fn new() -> Self {
        Self {
            global: GlobalScope::new(),
        }
    }

    pub fn load_stdlib(&mut self) -> anyhow::Result<()> {
        for module in read_stdlib()? {
            self.eval_module(&module).context("failed to eval module")?;
        }
        Ok(())
    }

    pub fn eval_module(&mut self, module: &Module) -> anyhow::Result<&Value> {
        for stmt in &module.stmts[..(module.stmts.len() - 1)] {
            self.eval_stmt(stmt)?;
        }
        let stmt = module.stmts.last().unwrap();
        self.eval_stmt(stmt)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<&Value> {
        match stmt {
            // Assignment: store the value in the global scope.
            Stmt::Assign { target, expr } => {
                let val = Value::from_expr(expr);
                let val = val.bind_global(&self.global);
                Ok(self.global.set(target, val))
            }
            // Variable name: show its value.
            Stmt::Expr {
                expr: Expr::Id { name },
            } => match self.global.get(name) {
                Some(val) => Ok(val),
                None => anyhow::bail!("variable `{name}` is not defined"),
            },
            // An arbitrary expression: eagerly evaluate.
            Stmt::Expr { expr } => {
                let val = Value::from_expr(expr);
                let val = val.bind_global(&self.global);
                let val = val.eval()?;
                Ok(self.global.set("_", val))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use rstest::rstest;

    #[rstest]
    #[case::id(r#"id"#, "λx x")]
    #[case::assign(r#"c = \a a a"#, "λa a a")]
    #[case::def(r#"\a id a"#, "λa id a")]
    #[case::call(r#"id A"#, "λa a")]
    #[case(r#"id B"#, "λb b")]
    #[case(r#"id A B"#, "λb b")]
    #[case(r#"(id A) B"#, "λb b")]
    #[case(r#"id (A B)"#, "λb b")]
    #[case::left(r#"(\a \b a) A B"#, "λa a")]
    #[case::right(r#"(\a \b b) A B"#, "λb b")]
    #[case(r#"(\a \a a) A B"#, "λb b")]
    #[case(r#"(\a (\a a) (\x a)) A"#, "λx a")]
    #[case(r#"(\a (\a a) (\x a)) A B"#, "λa a")]
    fn eval_module(#[case] input: &str, #[case] exp: &str) {
        let mut session = Session::new();
        session.eval_module(&parse("id = λx x").unwrap()).unwrap();
        session.eval_module(&parse("A = λa a").unwrap()).unwrap();
        session.eval_module(&parse("B = λb b").unwrap()).unwrap();

        let module = parse(input).unwrap();
        let val = session.eval_module(&module).unwrap();
        println!("Input: {input}");
        assert_eq!(val.repr(), exp);
    }
}
