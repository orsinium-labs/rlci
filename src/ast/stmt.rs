use super::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assign { target: String, expr: Box<Expr> },
    Expr { expr: Expr },
}

impl Stmt {
    pub fn short_repr(&self) -> String {
        match self {
            Stmt::Assign { target: _, expr } => format!("let({})", expr.short_repr()),
            Stmt::Expr { expr } => expr.short_repr(),
        }
    }
}
