//! ZECA - Zero Efficiency Compiler Augmented
//! Compiles a Rust-Like simplified language
//! Made for Compilers Assignment 2021/2 by prof. Dr. Eduardo Zambon
//! Using [Chumksy parser combinator](https://github.com/zesterer/chumsky)
//! Authors: @Henriquelay @AtilioA @meninadoukulele

#![warn(missing_docs)]

use chumsky::{prelude::*, Stream};

/// The AST for EXPR
#[derive(Debug)]
pub enum Expr {
    /// Numbers. `f64` should be the largest upcastable number and calculations will be done with these.
    Num(f64),

    /// Negation expression. Unary `-` operator.
    Neg(Box<Expr>),
    /// Addition expression. Binary `+` operator.
    Add(Box<Expr>, Box<Expr>),
    /// Subtaction expression. Binary `-` operator.
    Sub(Box<Expr>, Box<Expr>),
    /// Multiplication expression. Binary `*` operator.
    Mul(Box<Expr>, Box<Expr>),
    /// Division expression. Binary `/` operator.
    Div(Box<Expr>, Box<Expr>),

    /// Variables. Inside is stored the symbol name.
    Var(String),
    /// Function call expression. `()` operator placed after a symbol, as in `foo()`.
    Call(String, Vec<Expr>),
    /// Used to declare variables.
    Let {
        /// Name defined to this symbol.
        name: String,
        /// Value to be assigned to symbol.
        rhs: Box<Expr>,
        /// Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },
    /// Used to declare functions
    Fn {
        /// Name defined to this symbol.
        name: String,
        /// Arguments to be passed to function.
        args: Vec<String>,
        /// `Expr`s to be executed inside function.
        body: Box<Expr>,
        /// `Evaluated after symbol's own evaluation.
        then: Box<Expr>,
    },
}

/// Parses the program for correct tokens and tokens order.
pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let identifier = text::ident().padded();

    let expr = recursive(|expr| {
        // TODO for radix != 10, preceded by 0b, 0t, 0x
        let int = text::int(10)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded();

        let float = text::int::<_, Simple<char>>(10)
            .then_ignore(just('.'))
            .then(text::digits(10))
            .map(|s: (String, String)| Expr::Num(format!("{}.{}", s.0, s.1).parse().unwrap()))
            .padded();

        let number = float.or(int);

        let call = identifier
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing() // Foo is Rust-like, so allow trailing commas to appear in arg lists
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = number
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(identifier.map(Expr::Var));

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
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

        r#let.or(r#fn).or(expr).padded()
    });

    decl.then_ignore(end())
}

/// Evaluates `Expr`'s  return value.
fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(&'a String, f64)>,
    funcs: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
) -> Result<f64, String> {
    match expr {
        Expr::Num(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a, vars, funcs)?),
        Expr::Add(a, b) => Ok(eval(a, vars, funcs)? + eval(b, vars, funcs)?),
        Expr::Sub(a, b) => Ok(eval(a, vars, funcs)? - eval(b, vars, funcs)?),
        Expr::Mul(a, b) => Ok(eval(a, vars, funcs)? * eval(b, vars, funcs)?),
        Expr::Div(a, b) => Ok(eval(a, vars, funcs)? / eval(b, vars, funcs)?),
        Expr::Var(name) => {
            if let Some((_, val)) = vars.iter().rev().find(|(var, _)| *var == name) {
                Ok(*val)
            } else {
                Err(format!("Cannot find variable `{}` in scope", name))
            }
        }
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((name, rhs));
            let output = eval(then, vars, funcs);
            vars.pop();
            output
        }
        Expr::Call(name, args) => {
            if let Some((_, arg_names, body)) =
                funcs.iter().rev().find(|(var, _, _)| *var == name).copied()
            {
                if arg_names.len() == args.len() {
                    let mut args = args
                        .iter()
                        .map(|arg| eval(arg, vars, funcs))
                        .zip(arg_names.iter())
                        .map(|(val, name)| Ok((name, val?)))
                        .collect::<Result<_, String>>()?;
                    vars.append(&mut args);
                    let output = eval(body, vars, funcs);
                    vars.truncate(vars.len() - args.len());
                    output
                } else {
                    Err(format!(
                        "Wrong number of arguments for function `{}`: expected {}, found {}",
                        name,
                        arg_names.len(),
                        args.len(),
                    ))
                }
            } else {
                Err(format!("Cannot find function `{}` in scope", name))
            }
        }
        Expr::Fn {
            name,
            args,
            body,
            then,
        } => {
            funcs.push((name, args, body));
            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        }
    }
}

pub fn eval_value(src: String) -> Result<f64, Vec<String>> {
    match parser().parse(src) {
        Ok(ast) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => Ok(output),
            Err(eval_err) => Err(vec![format!("Evaluation error: {}", eval_err)]),
        },
        Err(parse_errs) => Err(parse_errs
            .into_iter()
            .map(|e| format!("Parse error: {}", e))
            .collect()),
    }
}
