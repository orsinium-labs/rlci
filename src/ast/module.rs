use super::stmt::Stmt;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub stmts: Vec<Stmt>,
}
