mod ast {
    mod expression;
    mod module;
    mod statement;

    pub use expression::Expression;
    pub use module::Module;
    pub use statement::Statement;
}

mod parser;
pub use parser::parse;
