use super::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assign { target: String, expr: Box<Expr> },
    Expr { expr: Expr },
}
