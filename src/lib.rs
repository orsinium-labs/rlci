extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_nodes;
mod parser;
pub use parser::{parse, LangParser};

mod interpreter {
    mod scope;
    mod value;
}
