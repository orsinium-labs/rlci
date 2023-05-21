use crate::ast::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<Module, Error<Rule>> {
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
    #[case::id(r#"id"#)]
    #[case::call(r#"id x"#)]
    #[case::def(r#"\x x"#)]
    #[case::assign(r#"id = \x x"#)]
    #[case::call_chain(r#"id a b"#)]
    #[case(r#"id = \a \b x"#)]
    #[case(r#"apply = \f f f"#)]
    #[case(r#"x = \f a (b c)"#)]
    #[case(r#"x = \f (a b) c"#)]
    #[case(r#"x = \f (\x x) c"#)]
    #[case(r#"x = \f (\x x)"#)]
    #[case(r#"+ a b"#)]
    #[case(r#"+ = \a \b a b"#)]
    #[case(r#"add = \a \b + a b"#)]
    #[case(r#"add = +"#)]
    fn smoke_parse_ok(#[case] input: &str) {
        assert!(parse(input).is_ok());
    }

    #[rstest]
    #[case(r#""#)]
    #[case(r#"\x"#)]
    #[case(r#"a \x"#)]
    #[case(r#"id = "#)]
    #[case(r#"id = \x"#)]
    fn smoke_parse_err(#[case] input: &str) {
        assert!(parse(input).is_err());
    }
}
