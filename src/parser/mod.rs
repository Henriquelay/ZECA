//! Parses input stream and outputs the corresponding ASTs. Mostly calls to 

use chumsky::{prelude::*, text::Character};

pub mod ast;

/// Parses inline and block comments
pub fn comment() -> impl Parser<char, (), Error = Simple<char>> {
    let block_comment = just::<_, _, Simple<char>>("/*").then(take_until(just("*/"))).ignored();
    let inline_comment = just::<_, _, Simple<char>>("//").then(take_until(text::newline())).ignored();
    block_comment.or(inline_comment)
}

/// Parses identifiers (variable names) as per [`chumsky::text::ident()`]
pub fn identifier() -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> {
    text::ident().padded()
}

/// Parses the program for correct tokens and tokens order.
pub fn parser() -> impl Parser<char, ast::Expr, Error = Simple<char>> {
    let identifier = text::ident().padded();

    let expr = recursive(|expr| {

        // TODO for radix != 10, preceded by 0b, 0t, 0x
        let integer = text::int(10)
            .map(|s: String| ast::Expr::Num(s.parse().unwrap()))
            .padded();

        let float = text::int::<_, Simple<char>>(10)
            .then_ignore(just('.'))
            .then(text::digits(10))
            .map(|s: (String, String)| ast::Expr::Num(format!("{}.{}", s.0, s.1).parse().unwrap()))
            .padded();

        let number = float.or(integer);

        let call = identifier
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing() // Foo is Rust-like, so allow trailing commas to appear in arg lists
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| ast::Expr::Call(f, args));

        let atom = number
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

        r#let.or(r#fn).or(expr).padded()
    });

    comment().padded().repeated().ignore_then(decl.then_ignore(end()))
    // comment.padded().repeated()
}
