use crate::ast_nodes::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

// This `derive` is the macro magic that generates the parser from the gramar file.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<Module, Error<Rule>> {
    // The `Rule` enum is generated from the grammar by the `derive(Parser)` above.
    // It lists all rules specified in the grammar.
    // The `parse` method accepts the root rule, which is the `module` in our case.
    //
    // Since the grammar for the module requires the module to have
    // at least one statement, we know that the Ok parse result has at least one item,
    // and so `next().unwrap()` will not panic. The pest documentation suggests
    // doing in the code this kind of assumptions based on the grammar.
    // This may still panic if the assumption I made is wrong (and it's still somehow
    // possible to get an empty module) or the grammar gets changed.
    //
    // If that worries you, you can handle such situations explicitly
    // or use [nom] instead which is a very type-safe library for writing parsers.
    // I used nom for the first implementation of the parser but then decided to
    // rewerite it to pest. The reason is that it's quite hard to work with nom,
    // resulting signatures are monstrouous, grammar is hard to read,
    // and error messages aren't particularly friendly.
    //
    // [nom]: https://github.com/rust-bakery/nom
    let root = LangParser::parse(Rule::module, input)?.next().unwrap();
    Ok(parse_module(root))
}

fn parse_module(root: Pair<Rule>) -> Module {
    let mut stmts: Vec<Stmt> = Vec::new();
    for subpair in root.into_inner() {
        if let Some(stmt) = parse_statement(subpair) {
            stmts.push(stmt)
        }
    }
    Module { stmts }
}

fn parse_statement(root: Pair<Rule>) -> Option<Stmt> {
    match root.as_rule() {
        Rule::statement => {
            let subpair = root.into_inner().next().unwrap();
            parse_statement(subpair)
        }
        Rule::assignment => {
            let mut subpairs = root.into_inner();
            let p1 = subpairs.next().unwrap();
            let p2 = subpairs.next().unwrap();
            Some(Stmt::Assign {
                target: p1.as_str().to_owned(),
                expr: Box::new(parse_expression(p2)),
            })
        }
        Rule::expression => {
            let subpair = root.into_inner().next().unwrap();
            let expr = parse_expression(subpair);
            Some(Stmt::Expr { expr })
        }
        Rule::EOI => None,
        _ => unreachable!(),
    }
}

fn parse_expression(root: Pair<Rule>) -> Expr {
    match root.as_rule() {
        Rule::expression => {
            let subpair = root.into_inner().next().unwrap();
            parse_expression(subpair)
        }
        Rule::definition => {
            let mut subpairs = root.into_inner();
            let p1 = subpairs.next().unwrap();
            let p2 = subpairs.next().unwrap();
            Expr::Def {
                arg: p1.as_str().to_owned(),
                expr: Box::new(parse_expression(p2)),
            }
        }
        Rule::call => {
            let mut subpairs = root.into_inner();
            let p1 = subpairs.next().unwrap();
            let mut target = parse_expression(p1);
            for p in subpairs {
                target = Expr::Call {
                    target: Box::new(target),
                    arg: Box::new(parse_expression(p)),
                }
            }
            target
        }
        Rule::identifier => Expr::Id {
            name: root.as_str().parse().unwrap(),
        },
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::id(r#"id"#, "id")]
    #[case::space(r#"  id"#, "id")]
    #[case::space(r#"id  "#, "id")]
    #[case::call(r#"id x"#, "call(id, id)")]
    #[case::def(r#"\x x"#, "def(id)")]
    #[case::def(r#"λx x"#, "def(id)")]
    #[case::assign(r#"id = \x x"#, "let(def(id))")]
    #[case::assign(r#"id = (\x x)"#, "let(def(id))")]
    #[case::assign(r#"& = \a a"#, "let(def(id))")]
    #[case(r#"id= \x x"#, "let(def(id))")]
    #[case(r#"id =\x x"#, "let(def(id))")]
    #[case(r#"id=\x x"#, "let(def(id))")]
    #[case::call_chain(r#"id a b"#, "call(call(id, id), id)")]
    #[case(r#"id = \a \b x"#, "let(def(def(id)))")]
    #[case(r#"apply = \f f f"#, "let(def(call(id, id)))")]
    #[case(r#"x = \f a (b c)"#, "let(def(call(id, call(id, id))))")]
    #[case(r#"x = \f (a b) c"#, "let(def(call(call(id, id), id)))")]
    #[case(r#"x = \f (\x x) c"#, "let(def(call(def(id), id)))")]
    #[case(r#"x = \f \x x"#, "let(def(def(id)))")]
    #[case(r#"x = \f (\x x)"#, "let(def(def(id)))")]
    #[case::call_punct(r#"+ a b"#, "call(call(id, id), id)")]
    #[case::assign_punct(r#"+ = \a \b a b"#, "let(def(def(call(id, id))))")]
    #[case(r#"add = \a \b + a b"#, "let(def(def(call(call(id, id), id))))")]
    #[case::alias(r#"add = +"#, "let(id)")]
    fn smoke_parse_stmt_ok(#[case] input: &str, #[case] exp: &str) {
        let module = parse(input).unwrap();
        assert_eq!(module.stmts.len(), 1);
        let stmt = &module.stmts[0];
        assert_eq!(stmt.short_repr(), exp);
    }

    #[rstest]
    #[case(r#""#)]
    #[case(r#"\x"#)]
    #[case(r#"a \x"#)]
    #[case(r#"id = "#)]
    #[case(r#"id = \x"#)]
    #[case(r#"(a)"#)]
    #[case(r#"(((\a a)))"#)]
    #[case(r#"\a a \b b a"#)]
    fn smoke_parse_stmt_err(#[case] input: &str) {
        assert!(parse(input).is_err());
    }
}
