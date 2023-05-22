#![allow(clippy::new_without_default)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_nodes;
mod parser;
pub use parser::{parse, LangParser};

pub mod interpreter {
    mod scope;
    mod session;
    mod value;

    pub use scope::{GlobalScope, LocalScope, Scope};
    pub use session::Session;
    pub use value::Value;
}
