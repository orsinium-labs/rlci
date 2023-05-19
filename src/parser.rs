use crate::ast::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many1;
use nom::sequence::{pair, preceded};
use nom::IResult;
use std::str;

fn parse_def<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    let p1 = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
    let p2 = preceded(char('\\'), p1);
    let p3 = pair(p2, parse_expr);
    map(p3, Expression::make_def)(i)
}

fn parse_call<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    let parser = pair(parse_expr, preceded(char(' '), parse_expr));
    map(parser, Expression::make_call)(i)
}

fn parse_assign<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    let p1 = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
    let parser = pair(p1, preceded(tag(" = "), parse_expr));
    map(parser, Expression::make_assign)(i)
}

fn parse_id<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    let parser = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
    map(parser, Expression::make_id)(i)
}

fn parse_expr<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    alt((parse_def, parse_call, parse_assign, parse_id))(i)
}

fn parse_statement<'a, E>(i: &'a str) -> IResult<&'a str, Statement, E>
where
    E: ParseError<&'a str>,
{
    let parser = many1(parse_def);
    map(parser, |x| Statement { expressions: x })(i)
}

fn parse_module<'a, E>(i: &'a str) -> IResult<&'a str, Module, E>
where
    E: ParseError<&'a str>,
{
    let parser = many1(parse_statement);
    map(parser, |x| Module { statements: x })(i)
}

pub fn parse(input: &str) -> Result<(&str, Module), nom::Err<(&str, ErrorKind)>> {
    parse_module::<(&str, ErrorKind)>(input)
}
