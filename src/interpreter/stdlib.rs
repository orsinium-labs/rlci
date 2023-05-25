use crate::ast_nodes::Module;
use crate::parse;
use anyhow::Context;
use include_dir::{include_dir, Dir};

static STDLIB_DIR: Dir = include_dir!("./src/stdlib");

pub fn read_stdlib() -> anyhow::Result<Vec<Module>> {
    let mut modules: Vec<Module> = Vec::new();
    for file in STDLIB_DIR.files() {
        let content = file.contents_utf8().unwrap();
        let emsg = format!("failed to parse {:?} module", file.path());
        let module = parse(content).context(emsg)?;
        modules.push(module);
    }
    Ok(modules)
}

#[cfg(test)]
mod tests {
    use crate::interpreter::*;
    use crate::parse;
    use rstest::rstest;

    const T: &str = "λa λb a";
    const F: &str = "λa λb b";

    #[rstest]
    #[case::id(r#"id"#, "λa a")]
    // test bool
    #[case::true_repr(r#"true"#, T)]
    #[case::false_repr(r#"false"#, F)]
    #[case::not(r#"not true"#, F)]
    #[case::not(r#"not false"#, T)]
    #[case::and(r#"and false false"#, F)]
    #[case::and(r#"and false true"#, F)]
    #[case::and(r#"and true false"#, F)]
    #[case::and(r#"and true true"#, T)]
    #[case::or(r#"or false false"#, F)]
    #[case::or(r#"or false true"#, T)]
    #[case::or(r#"or true false"#, T)]
    #[case::or(r#"or true true"#, T)]
    #[case::xor(r#"xor false false"#, F)]
    #[case::xor(r#"xor false true"#, T)]
    #[case::xor(r#"xor true false"#, T)]
    #[case::xor(r#"xor true true"#, F)]
    #[case::xnor(r#"xnor false false"#, T)]
    #[case::xnor(r#"xnor false true"#, F)]
    #[case::xnor(r#"xnor true false"#, F)]
    #[case::xnor(r#"xnor true true"#, T)]
    fn stdlib(#[case] input: &str, #[case] exp: &str) {
        let hinter = AutoCompleter::new();
        let mut session = Session::new(&hinter);
        session.load_stdlib().unwrap();
        let module = parse(input).unwrap();
        let val = session.eval_module(&module).unwrap();
        println!("Input: {input}");
        assert_eq!(val.repr(), exp);
    }
}
