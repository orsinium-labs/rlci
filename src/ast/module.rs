use super::statement::Stmt;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}
