#![doc = include_str!("../README.md")]
#![allow(clippy::new_without_default)]
#![deny(missing_docs)]

// Makes it possible to use pest parser, must be defined at the crate level.
extern crate pest;
#[macro_use]
extern crate pest_derive;

// The parser, converts the plain text input into AST nodes.
mod ast_nodes;
mod parser;
pub use parser::{parse, LangParser};

/// The interpreter, evaluates values at runtime.
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
