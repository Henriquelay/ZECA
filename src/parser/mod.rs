//! Parses input stream and outputs the corresponding ASTs. Mostly calls to

use chumsky::{prelude::*, text::Character};

pub mod ast;

/// Parses a single inline or block comment
pub fn comment() -> impl Parser<char, (), Error = Simple<char>> + Copy + Clone {
    let inline_comment = just("//")
        .ignore_then(take_until(text::newline()))
        .ignored();
    let single_block_comment = just("/*").ignore_then(take_until(just("*/"))).ignored();
    single_block_comment.or(inline_comment)
}

/// Parses identifiers (variable/function names), defined as per [`chumsky::text::ident()`]
pub fn identifier(
) -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> + Copy + Clone {
    text::ident().padded()
}

/// Parses an integer number of radix 10
/// TODO for radix != 10, preceded by 0b, 0t, 0x
pub fn integer() -> impl Parser<char, ast::Expr, Error = Simple<char>> + Copy + Clone {
    text::int(10)
    .map(|s: String| ast::Expr::Num(ast::Number::Integer(s.parse().unwrap())))
    .padded()
}

/// Parses a floating-point number
/// TODO scientific notation
pub fn float() -> impl Parser<char, ast::Expr, Error = Simple<char>> + Copy + Clone {
    text::int::<_, Simple<char>>(10)
    .then_ignore(just('.'))
    .then(text::digits(10).or_not())
    .map(|s: (String, Option<String>)| {
        ast::Expr::Num(ast::Number::Float(
            format!("{}.{}", s.0, s.1.unwrap_or("".to_string())).parse().unwrap(),
        ))
    })
    .padded()
}

/// Any number. Ints or floats.
pub fn number() -> impl Parser<char, ast::Expr, Error = Simple<char>> + Copy + Clone {
    let number = float().or(integer());
    number
}

/// True of false. Rejects on anything else
pub fn boolean() -> impl Parser<char, ast::Expr, Error = Simple<char>> + Copy + Clone {
    just("true")
        .or(just("false"))
        .map(|s| ast::Expr::Bool(s.parse().unwrap()))
}

/// Parses the string type. Does not support escaping.
pub fn string() -> impl Parser<char, ast::Expr, Error = Simple<char>> + Copy + Clone{
    just('"')
        .ignore_then(take_until(just('"')))
        // .collect::<String>()
        .map(|_ | ast::Expr::Bool(true))
}

/// Parses the program for correct tokens and tokens order.
/// Finished parsers are stored into variables and no call should be made to the variable itself, only chaining methods.
/// Should NOT expect any kind of end-of-file ([`end()`][chumsky::prelude::end()]), as it will interfere with unitary tests and instead should be prepended when [`parser.parse()`][chumsky::Parser::parse()] is called, usually with `then_ignore(end())`.
pub fn parser() -> impl Parser<char, ast::Expr, Error = Simple<char>> {
    let comment = comment();

    let identifier = identifier();

    let string = string();

    let expr = recursive(|expr| {
        let number = number();
        let boolean = boolean();

        let call = identifier
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing() // Foo is Rust-like, so allow trailing commas to appear in arg lists
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| ast::Expr::Call(f, args));

        let atom = number
            .or(string)
            .or(boolean)
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(identifier.map(ast::Expr::Var));

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| ast::Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(ast::Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(ast::Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(ast::Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(ast::Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum.padded()
    });

    let decl = recursive(|decl| {
        let r#let = text::keyword("let")
            .ignore_then(identifier)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl.clone())
            .map(|((name, rhs), then)| ast::Expr::Let {
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
            .map(|(((name, args), body), then)| ast::Expr::Fn {
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
