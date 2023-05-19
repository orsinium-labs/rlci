use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub expressions: Vec<Expression>,
}
