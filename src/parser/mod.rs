//! Parses input stream and outputs the corresponding ASTs. Mostly calls to

use chumsky::{prelude::*, text::Character};

pub mod ast;
use ast::*;

/// Parses a single inline or block comment
pub fn comment_parser() -> impl Parser<char, (), Error = Simple<char>> + Clone + Copy {
    let inline_comment = just("//")
        .ignore_then(take_until(text::newline()))
        .ignored();
    let single_block_comment = just("/*").ignore_then(take_until(just("*/"))).ignored();
    // let nested_block_comment = recursive(|nested_comment| {
    //     single_block_comment.or(single_block_comment.delimited_by(nested_comment.clone(), nested_comment))
    // }).ignored();
    single_block_comment.or(inline_comment)
}

/// Parses identifiers (variable/function names), defined as per [`chumsky::text::ident()`]
pub fn identifier_parser(
) -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> + Copy + Clone {
    text::ident().padded()
}

/// Parses an integer number of radix 10
/// TODO for radix != 10, preceded by 0b, 0t, 0x
pub fn integer_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    text::int(10)
        .map(|s: String| Expr::Lit(Literal::Num(Number::Integer(s.parse().unwrap()))))
        .padded()
}

/// Parses a floating-point number
/// TODO scientific notation
pub fn float_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    text::int::<_, Simple<char>>(10)
        .then_ignore(just('.'))
        .then(text::digits(10).or_not())
        .map(|s: (String, Option<String>)| {
            Expr::Lit(Literal::Num(Number::Float(
                format!("{}.{}", s.0, s.1.unwrap_or("".to_string()))
                    .parse()
                    .unwrap(),
            )))
        })
        .padded()
}

/// Any number. Ints or floats.
pub fn number_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    let number = float_parser().or(integer_parser());
    number
}

/// True of false. Rejects on anything else
pub fn boolean_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    just("true")
        .or(just("false"))
        .map(|s| Expr::Lit(Literal::Bool(s.parse().unwrap())))
}

/// Parses the string type. Does not support escaping.
pub fn string_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    just('"')
        .ignore_then(take_until(just('"')))
        // .collect::<String>()
        .map(|_| Expr::Lit(Literal::Bool(true)))
}

/// Parses expressions, made of `atom`s
pub fn expr_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Clone {
    let identifier = identifier_parser();

    let string = string_parser();
    let number = number_parser();
    let boolean = boolean_parser();
    recursive(|expr| {
        let call = identifier
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing() // Foo is Rust-like, so allow trailing commas to appear in arg lists
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = number
            .or(string)
            .or(boolean)
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(identifier.map(Expr::Var));

        let op = |c| just(c).padded();

        let unary = op("-")
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op("*")
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op("/").to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op("+")
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op("-").to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let comparation = sum
            .clone()
            .then(
                op("==")
                    .to(Expr::Eq as fn(_, _) -> _)
                    .or(op("<").to(Expr::Lt as fn(_, _) -> _))
                    .or(op(">").to(Expr::Gt as fn(_, _) -> _))
                    .then(sum)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        comparation.padded()
    })
}

/// Parses the program for correct tokens and tokens order.
/// Finished parsers are stored into variables and no call should be made to the variable itself, only chaining methods.
/// Should NOT expect any kind of end-of-file ([`end()`][chumsky::prelude::end()]), as it will interfere with unitary tests and instead should be prepended when [`parser.parse()`][chumsky::Parser::parse()] is called, usually with `then_ignore(end())`.
pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let comment = comment_parser();

    let identifier = identifier_parser();

    let expr = expr_parser();

    let decl = recursive(|decl| {
        let r#let = text::keyword("let")
            .ignore_then(identifier)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl.clone())
            .map(|((name, rhs), then)| Expr::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        let r#fn = text::keyword("fn")
            .ignore_then(identifier)
            .then(identifier.repeated())
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl)
            .map(|(((name, args), body), then)| Expr::Fn {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then),
            });

        r#let
            .or(r#fn)
            .or(expr)
            .padded_by(comment.padded().repeated())
            .padded()
    });

    decl
}
