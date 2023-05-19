use super::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub statements: Vec<Statement>,
}
