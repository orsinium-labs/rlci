use crate::ast::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

#[allow(clippy::result_large_err)]
pub fn parse(input: &str) -> Result<Module, Error<Rule>> {
    let root = LangParser::parse(Rule::module, input)?
        .next()
        .expect("there is only one root");
    Ok(parse_module(root))
}

fn parse_module(root: Pair<Rule>) -> Module {
    let mut stmts: Vec<Stmt> = Vec::new();
    for subpair in root.into_inner() {
        if let Some(stmt) = parse_statement(subpair) {
            stmts.push(stmt)
        }
    }
    Module { statements: stmts }
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
            let p2 = subpairs.next().unwrap();
            Expr::Call {
                target: Box::new(parse_expression(p1)),
                arg: Box::new(parse_expression(p2)),
            }
        }
        Rule::identifier => Expr::Id {
            name: root.as_str().parse().unwrap(),
        },
        _ => unreachable!(),
    }
}
