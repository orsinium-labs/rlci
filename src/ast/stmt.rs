use super::expr::Expr;

/// Statement is a node that can be used only on the module-level.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// Assign an expression to a name.
    ///
    /// `id = \x x`
    Assign { target: String, expr: Box<Expr> },

    /// A single module-level expression.
    /// It doesn't make sense in the real-world scripts
    /// but essential for REPL.
    Expr { expr: Expr },
}

impl Stmt {
    /// Represent the statement as a short S-tree-like string that reflects
    /// the AST structure but not the content of the statement.
    ///
    /// It is very helpful for testing and debugging the parser, especially to find
    /// inconsistent or unexpected results in ambiguous situations.
    pub fn short_repr(&self) -> String {
        match self {
            Stmt::Assign { target: _, expr } => format!("let({})", expr.short_repr()),
            Stmt::Expr { expr } => expr.short_repr(),
        }
    }
}
