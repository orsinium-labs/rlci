#[derive(Debug, PartialEq)]
pub enum Expr {
    Def { arg: String, expr: Box<Expr> },
    Call { target: Box<Expr>, arg: Box<Expr> },
    Id { name: String },
}
