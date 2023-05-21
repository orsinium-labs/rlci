#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Definition of a lambda.
    Def { arg: String, expr: Box<Expr> },

    /// Application. Calling a lambda with an argument.
    Call { target: Box<Expr>, arg: Box<Expr> },

    /// Identifier, a name of a lambda.
    Id { name: String },
}
