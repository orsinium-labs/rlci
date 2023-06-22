use crate::ast_nodes::{Expr, Module, Stmt};
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

/// The parser, knows how to parse th language syntax using the grammar file.

// This `derive` is the macro magic that generates the parser from the gramar file.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

/// Convert a plain text input into AST nodes.
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
    // The Pair.into_inner method returns an iterator over the rules
    // inside of the given rule. In this case, it iterates over statements
    // extracted by `statement+` part of the `module` rule.
    let stmts: Vec<Stmt> = root.into_inner().filter_map(parse_statement).collect();
    Module { stmts }
}

fn parse_statement(root: Pair<Rule>) -> Option<Stmt> {
    match root.as_rule() {
        Rule::statement => {
            // The statement includes only one subpair, either an assignment
            // or an expression. We parse it recursively.
            let subpair = root.into_inner().next().unwrap();
            parse_statement(subpair)
        }

        Rule::assignment => {
            // The assignment rule has exactly 2 pairs: the target and the expression.
            let mut subpairs = root.into_inner();
            let p1 = subpairs.next().unwrap();
            let p2 = subpairs.next().unwrap();
            Some(Stmt::Assign {
                // I clone the string here because I don't want to deal with lifetimes.
                // Otherwise, we'd have to ensure that the user input is lives as long
                // (or longer) as the parsed AST.
                target: p1.as_str().to_string(),
                expr: Box::new(parse_expression(p2)),
            })
        }

        Rule::expression => {
            // `parse_expression` knows how to parse the `expression` rule,
            // so you could just use `parse_expression(root)` instead,
            // and it would work just fine. Unwrapping it here is an optimization
            // to have one fewer recursive function call.
            let subpair = root.into_inner().next().unwrap();
            let expr = parse_expression(subpair);
            Some(Stmt::Expr { expr })
        }

        // For some reason, pest included EOI in the list of generated rules
        // as soon as I wrapped it into `()`. The presence of this rule
        // is the sole reason why `parse_statement` returns and `Option`.
        //
        // It's possible that some other cases in the future will also return `None`.
        // For example, if you consider comments a statement and don't silence the token.
        Rule::EOI => None,

        // If you don't add an explicit catch-all branch for `match`,
        // Rust will report that the match isn't exhaustive because `Rule`
        // includes all the rules in the grammar, while `parse_statement`
        // handles only the rules inside of the `statement` rule.
        // Similar to `unwrap` cases in this file, this may panic
        // if the grammar is updated or the assumption about the grammar is wrong.
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
            root.into_inner()
                .map(parse_expression)
                .reduce(|target, arg| Expr::Call {
                    target: Box::new(target),
                    arg: Box::new(arg)
                })
                .unwrap()
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
