use crate::ast_nodes::Module;
use crate::parse;
use anyhow::Context;
use include_dir::{include_dir, Dir};

static STDLIB_DIR: Dir = include_dir!("./src/stdlib");

pub fn read_stdlib() -> anyhow::Result<Vec<Module>> {
    let mut modules: Vec<Module> = Vec::new();
    for file in STDLIB_DIR.files() {
        let content = file.contents_utf8().unwrap();
        let module = parse(content).context("failed to parse module")?;
        modules.push(module);
    }
    Ok(modules)
}
