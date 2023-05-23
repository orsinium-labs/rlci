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
                let val = val.bind(&self.global);
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
                let val = val.bind(&self.global);
                Ok(self.global.set("_", val))
            }
        }
    }
}
