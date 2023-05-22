use crate::ast_nodes::{Module, Stmt};
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

    pub fn eval_module<'a>(&'a mut self, module: &Module) -> &'a Value {
        for stmt in &module.stmts[..(module.stmts.len() - 1)] {
            self.eval_stmt(stmt);
        }
        let stmt = module.stmts.last().unwrap();
        self.eval_stmt(stmt)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> &Value {
        match stmt {
            Stmt::Assign { target, expr } => {
                let val = Value::from_expr(expr);
                let val = val.bind(&self.global);
                self.global.set(target, val);
                self.global.get(target).unwrap()
            }
            Stmt::Expr { expr } => {
                let val = Value::from_expr(expr);
                let val = val.bind(&self.global);
                self.global.set("_", val);
                self.global.get("_").unwrap()
            }
        }
    }
}
