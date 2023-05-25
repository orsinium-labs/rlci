#![allow(clippy::new_without_default)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_nodes;
mod parser;
pub use parser::{parse, LangParser};

pub mod interpreter {
    mod hints;
    mod repl;
    mod scope;
    mod session;
    mod stdlib;
    mod value;

    pub use hints::LangHinter;
    pub use repl::run_repl;
    pub use scope::{GlobalScope, LocalScope};
    pub use session::Session;
    pub use stdlib::read_stdlib;
    pub use value::Value;
}
