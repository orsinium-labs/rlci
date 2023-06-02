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
    // bool
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
    // nat operations
    #[case::inc(r#"eq (inc 0) 1"#, T)]
    #[case::inc(r#"eq (inc 2) 3"#, T)]
    #[case::add(r#"eq (add 2 3) 5"#, T)]
    #[case::mul(r#"eq (mul 2 3) 6"#, T)]
    #[case::pow(r#"eq (pow 3 2) 9"#, T)]
    #[case::dec(r#"eq (dec 3) 2"#, T)]
    #[case::dec(r#"eq (dec 0) 0"#, T)]
    #[case::sub(r#"eq (sub 5 3) 2"#, T)]
    #[case::sub(r#"eq (sub 3 5) 0"#, T)]
    #[case::diff(r#"eq (diff 5 3) 2"#, T)]
    #[case::diff(r#"eq (diff 3 5) 2"#, T)]
    #[case::min(r#"eq (min 3 5) 3"#, T)]
    #[case::min(r#"eq (min 5 3) 3"#, T)]
    #[case::max(r#"eq (max 3 5) 5"#, T)]
    #[case::max(r#"eq (max 5 3) 5"#, T)]
    // nat comparison
    #[case::is_zero(r#"is_zero 0"#, T)]
    #[case::is_zero(r#"is_zero 1"#, F)]
    #[case::eq(r#"eq 2 2"#, T)]
    #[case::eq(r#"eq 2 3"#, F)]
    #[case::eq(r#"eq 3 2"#, F)]
    #[case::lt(r#"lt 2 5"#, T)]
    #[case::lt(r#"lt 5 2"#, F)]
    #[case::lt(r#"lt 2 2"#, F)]
    #[case::gt(r#"gt 2 5"#, F)]
    #[case::gt(r#"gt 5 2"#, T)]
    #[case::gt(r#"gt 2 2"#, F)]
    #[case::lte(r#"lte 2 5"#, T)]
    #[case::lte(r#"lte 5 2"#, F)]
    #[case::lte(r#"lte 2 2"#, T)]
    #[case::gte(r#"gte 2 5"#, F)]
    #[case::gte(r#"gte 5 2"#, T)]
    #[case::gte(r#"gte 2 2"#, T)]
    // rec
    #[case::fac(r#"eq (fac 2) 2"#, T)]
    #[case::fac(r#"eq (fac 3) 6"#, T)]
    // #[case::fac(r#"eq (fac 4) (mul 6 4)"#, T)]
    #[case::fib(r#"eq (fib 1) 1"#, T)]
    #[case::fib(r#"eq (fib 2) 1"#, T)]
    #[case::fib(r#"eq (fib 3) 2"#, T)]
    #[case::fib(r#"eq (fib 4) 3"#, T)]
    #[case::fib(r#"eq (fib 5) 5"#, T)]
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
