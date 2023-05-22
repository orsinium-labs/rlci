/// Expression is a node that can be used anywhere.
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Definition of a lambda.
    Def { arg: String, expr: Box<Expr> },

    /// Application. Calling a lambda with an argument.
    Call { target: Box<Expr>, arg: Box<Expr> },

    /// Identifier, a name of a lambda.
    Id { name: String },
}

impl Expr {
    pub fn short_repr(&self) -> String {
        match self {
            Expr::Def { arg: _, expr } => format!("def({})", expr.short_repr()),
            Expr::Call { target, arg } => {
                format!("call({}, {})", target.short_repr(), arg.short_repr())
            }
            Expr::Id { name: _ } => "id".to_string(),
        }
    }
}
