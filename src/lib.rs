#![allow(clippy::new_without_default)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_nodes;
mod parser;
pub use parser::{parse, LangParser};

pub mod interpreter {
    mod helper;
    mod repl;
    mod scope;
    mod session;
    mod stdlib;
    mod value;

    pub(crate) use helper::Helper;
    pub use repl::run_repl;
    pub(crate) use scope::GlobalScope;
    pub use session::Session;
    pub(crate) use stdlib::read_stdlib;
    pub(crate) use value::Value;
}
