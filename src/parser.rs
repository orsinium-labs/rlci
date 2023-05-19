use crate::ast::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::error::ParseError;
use nom::multi::many1;
use nom::sequence::{delimited, pair, preceded};
use nom::IResult;
use std::str;

fn drop_spaces<'a, E>(i: &'a str) -> IResult<&'a str, &str, E>
where
    E: ParseError<&'a str>,
{
    take_while(|c: char| c.is_ascii_whitespace())(i)
}

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

pub fn parse_expr<'a, E>(i: &'a str) -> IResult<&'a str, Expression, E>
where
    E: ParseError<&'a str>,
{
    alt((parse_def, parse_call, parse_assign, parse_id))(i)
}

fn parse_statement<'a, E>(i: &'a str) -> IResult<&'a str, Statement, E>
where
    E: ParseError<&'a str>,
{
    let parser = many1(parse_assign);
    map(parser, |x| Statement { expressions: x })(i)
}

pub fn parse_module<'a, E>(i: &'a str) -> IResult<&'a str, Module, E>
where
    E: ParseError<&'a str>,
{
    let p1 = delimited(drop_spaces, parse_statement, opt(drop_spaces));
    let parser = many1(p1);
    map(parser, |x| Module { statements: x })(i)
}
