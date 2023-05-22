extern crate pest;
#[macro_use]
extern crate pest_derive;

/// Definitions for AST nodes.
mod ast {
    mod expr;
    mod module;
    mod stmt;

    pub use expr::Expr;
    pub use module::Module;
    pub use stmt::Stmt;
}

mod parser;
pub use parser::{parse, LangParser};
