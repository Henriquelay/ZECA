//! Parses input stream and outputs the corresponding ASTs. Mostly calls to

use chumsky::{prelude::*, text::Character};

pub mod ast;
use ast::*;

// Terminals (not made from other types) {

/// Parses a single inline or block comment.
pub fn comment_parser() -> impl Parser<char, (), Error = Simple<char>> + Clone + Copy {
    // Parse "//"
    let inline_comment = just("//")
        // Ignore input until newline
        .ignore_then(take_until(text::newline()))
        // Don't allocate memory for this
        .ignored();
    let single_block_comment = just("/*").ignore_then(take_until(just("*/"))).ignored();

    // let nested_block_comment = recursive(|nested_comment| {
    //     single_block_comment.or(single_block_comment.delimited_by(nested_comment.clone(), nested_comment))
    // }).ignored();

    // Parse block or inline comments
    single_block_comment.or(inline_comment)
}

// TODO[epic-unicode] Update to support unicode XID.
/// Parses identifiers (variable/function names), defined as per [`chumsky::text::ident()`].
pub fn identifier_parser(
) -> impl Parser<char, <char as Character>::Collection, Error = Simple<char>> + Copy + Clone {
    text::ident().padded()
}

/// Parses an integer number of radix 10.
/// TODO for radix != 10, preceded by 0b, 0t, 0x.
pub fn integer_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    // Parse for base 10
    text::int(10)
        .map(|s: String| Expr::Literal(Literal::Num(Number::Integer(s.parse().unwrap()))))
        .padded()
}

/// Parses a floating-point number.
/// TODO[epic-scientific-notation] parse scientific notation.
pub fn float_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    // Try to match a integer, then a dot, then another series of digits
    text::int::<_, Simple<char>>(10)
        .then_ignore(just('.'))
        .then(text::digits(10).or_not())
        .map(|s: (String, Option<String>)| {
            Expr::Literal(Literal::Num(Number::Float(
                // The number after the dot can be omitted (e.g.: "2." is a float)
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
        .map(|s| Expr::Literal(Literal::Bool(s.parse().unwrap())))
}

/// Parses the string type. Does not support escaping.
pub fn string_parser() -> impl Parser<char, Expr, Error = Simple<char>> + Copy + Clone {
    just('"')
        .ignore_then(take_until(just('"')))
        // .collect::<String>()
        .map(|_| Expr::Literal(Literal::Bool(true)))
}

// }
// Non-terminal (Composite types) {

/// A statement is a component of a block, which is in turn a component of an outer expression or function.
pub fn statement_parser() -> impl Parser<char, Statement, Error = Simple<char>> + Clone {
    let identifier = identifier_parser();
    // TODO nested items
    // let item = item_parser().map(|s| Statement::Item(Box::new(s)));
    let expr = expr_parser();
    let r#let = text::keyword("let")
        .ignore_then(identifier)
        .then_ignore(just('='))
        .then(expr.clone())
        .map(|(name, rhs)| Statement::Let {
            name,
            rhs: Box::new(rhs),
        });
    r#let
        // .or(item)
        .or(expr.map(|s| Statement::Expr(s)))
        .or_not()
        .then_ignore(just(";"))
        .map(|s| match s {
            Some(x) => x,
            None => Statement::Null,
        })
}

/// Parses a
pub fn block_parser() -> impl Parser<char, Block, Error = Simple<char>> + Clone {
    let statement = statement_parser();
    let comment = comment_parser();
    statement
        .padded_by(comment.padded().repeated())
        .padded()
        .repeated()
        .delimited_by(just("{"), just("}"))
        .map(|s| Block(s))
}

/// An item is a component of a crate. Items are organized within a crate by a nested set of modules. Every crate has a single "outermost" anonymous module; all further items within the crate have paths within the module tree of the crate.
/// A function consists of a block, along with a name and a set of parameters. Other than a name, all these are optional. Functions are declared with the keyword fn. Functions may declare a set of input variables as parameters, through which the caller passes arguments into the function, and the output type of the value the function will return to its caller on completion.
pub fn item_parser() -> impl Parser<char, Item, Error = Simple<char>> + Clone {
    let identifier = identifier_parser();
    let block = block_parser();
    let function = text::keyword("fn")
        .ignore_then(identifier)
        .then_ignore(just("("))
        .then(identifier.then_ignore(just(',').or_not()).repeated())
        .then_ignore(just(")"))
        .then(block.padded())
        .map(|((name, args), body)| {
            Item::Function(Function {
                name,
                args,
                body: Box::new(body),
            })
        });
    function
}

/// Parses expressions, made of `atom`s.
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
                    // Allow trailing commas to appear in arg lists
                    .allow_trailing()
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
// }

/// Parses the whole program for correct tokens and tokens order.
/// Finished parsers are stored into variables and no call should be made to the variable itself, only chaining methods.
/// Should NOT expect any kind of end-of-file ([`end()`][chumsky::prelude::end()]), as it will interfere with unitary tests and instead should be prepended when [`parser.parse()`][chumsky::Parser::parse()] is called, usually with `then_ignore(end())`.
/// Returns all items (top level constructs).
pub fn parser() -> impl Parser<char, Vec<Item>, Error = Simple<char>> {
    let comment = comment_parser();
    let item = item_parser();

    item.padded_by(comment.padded().repeated())
        .repeated()
        .padded()
}
