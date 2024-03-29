use anyhow::Context;

use crate::ast_nodes::{Expr, Module, Stmt};
use crate::interpreter::{read_stdlib, GlobalScope, Helper, Value};

/// Session is a runtime of a program.
///
/// The Session loads stdlib, holds the global state, and is responsible
/// for evaluating modules and statements.
pub struct Session<'a> {
    global: GlobalScope,
    helper: Option<&'a Helper>,
}

impl<'a> Session<'a> {
    /// Create a new Session instance.
    ///
    /// Session may optionally accept a pointer to a helper.
    /// If available, it will update the helpers with the list of
    /// new global names defined.
    #[must_use]
    pub fn new(helper: Option<&'a Helper>) -> Self {
        Self {
            global: GlobalScope::new(),
            helper,
        }
    }

    /// Read stdlib and evaluate it in the current session context.
    ///
    /// Since there is no "import" statement, users cannot explicitly
    /// import anything from the stdlib, so we implicitly import it
    /// at the shell startup. It's called "prelude" in Haskell and Rust
    /// and "builtins" in Python.
    pub fn load_stdlib(&mut self) -> anyhow::Result<()> {
        for module in read_stdlib()? {
            self.eval_module(&module).context("failed to eval module")?;
        }
        Ok(())
    }

    /// Evaluate all statements in the module and return the result of the last one.
    pub fn eval_module(&mut self, module: &Module) -> anyhow::Result<&Value> {
        for stmt in &module.stmts[..(module.stmts.len() - 1)] {
            self.eval_stmt(stmt)?;
        }
        let stmt = module.stmts.last().unwrap();
        self.eval_stmt(stmt)
    }

    /// Evaluate the statement and return its result.
    ///
    /// In the current implementation, any statement can be evaluated
    /// into a specific value. The only failure possible is when a name is undefined.
    fn eval_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<&Value> {
        match stmt {
            // Assignment: store the value in the global scope.
            Stmt::Assign { target, expr } => {
                let val = Value::from_expr(expr);
                let val = val.bind_global(&self.global);
                if let Some(helper) = self.helper {
                    helper.add(target);
                }
                Ok(self.global.set(target, val))
            }
            // Variable name: show its value.
            //
            // The `repr` shows global bound variables by their name,
            // which is great for more concise representation of complex
            // expressions but quite useless for when the whole expression
            // is just the name.
            // Without this branch, when the user types `true`, they would simply get
            // `true` back. This branch unwraps the explicit name to the actual
            // representation of its value.
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

    // These are the most important tests of the runtime.
    // Perhaps, there should be more of these.
    #[rstest]
    #[case::id(r"id", "λx x")]
    #[case::assign(r"c = \a a a", "λa a a")]
    #[case::def(r"\a id a", "λa id a")]
    #[case::call(r"id A", "λa a")]
    #[case(r"id B", "λb b")]
    #[case(r"id A B", "λb b")]
    #[case(r"(id A) B", "λb b")]
    #[case(r"id (A B)", "λb b")]
    #[case::left(r"(\a \b a) A B", "λa a")]
    #[case::right(r"(\a \b b) A B", "λb b")]
    #[case(r"(\a \a a) A B", "λb b")]
    #[case(r"(\a (\a a) (\x a)) A", "λx A")]
    #[case(r"(\a (\a a) (\x a)) A B", "λa a")]
    #[case(r"(\a \a a) A", "λa a")]
    fn eval_module(#[case] input: &str, #[case] exp: &str) {
        let mut session = Session::new(None);
        session.eval_module(&parse("id = λx x").unwrap()).unwrap();
        session.eval_module(&parse("A = λa a").unwrap()).unwrap();
        session.eval_module(&parse("B = λb b").unwrap()).unwrap();

        let module = parse(input).unwrap();
        let val = session.eval_module(&module).unwrap();
        println!("Input: {input}");
        assert_eq!(val.repr(), exp);
    }
}
