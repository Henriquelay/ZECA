//! # Zero Efficiency Compiler Augmented
//! ## ![Zequinha](https://lh3.googleusercontent.com/a-/AOh14GhBKpwRTXOjbn0kJ0-PtHSxdeRzpop431D_J9c_=s32-c) Compiles a Rust-Like simplified language  
//! Using [Chumksy parser combinator](https://github.com/zesterer/chumsky)  
//! ---
//! > _Made for Compilers Assignment 2021/2 by prof. Dr. Eduardo Zambon_  
//! > _Authors: [@Henriquelay](https://github.com/Henriquelay) [@AtilioA](https://github.com/AtilioA) [@luanagabrielescosta](https://github.com/luanagabrielescosta)_

#![warn(missing_docs)]

#[cfg(test)]
mod unittest;

pub mod parser;
use chumsky::{prelude::end, Parser};
use parser::{ast::*, parser};

macro_rules! for_every_number_Value {
    ($expr:expr, $clj:expr) => {
        match $expr {
            (Literal::Num(n), Literal::Num(o)) => match (n, o) {
                (Number::Integer(x), Number::Integer(y)) => $clj(x, y),
                (Number::UInteger(x), Number::UInteger(y)) => $clj(x, y),
                (Number::Float(x), Number::Float(y)) => $clj(x, y),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    };
}

macro_rules! for_every_number_Value_wrapped {
    ($expr:expr, $clj:expr) => {
        match $expr {
            (Literal::Num(n), Literal::Num(o)) => match (n, o) {
                (Number::Integer(x), Number::Integer(y)) => Number::Integer($clj(x, y)),
                (Number::UInteger(x), Number::UInteger(y)) => Number::UInteger($clj(x, y)),
                (Number::Float(x), Number::Float(y)) => Number::Float($clj(x, y)),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    };
}

/// Evaluates `Expr`'s  return value.
fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(String, Literal)>,
    funcs: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
) -> Result<Literal, String> {
    match expr {
        Expr::Lit(x) => Ok(*x),
        Expr::Lt(a, b) => Ok(Literal::Bool({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x < y)
        })),
        Expr::Gt(a, b) => Ok(Literal::Bool({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x > y)
        })),
        Expr::Eq(a, b) => Ok(Literal::Bool({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x == y)
        })),
        Expr::Neg(a) => match eval(a, vars, funcs)? {
            Literal::Num(x) => Ok(Literal::Num(-x)),
            Literal::Bool(x) => Ok(Literal::Bool(!x)),
        },
        Expr::Add(a, b) => Ok(Literal::Num({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x + y)
        })),
        Expr::Sub(a, b) => Ok(Literal::Num({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x - y)
        })),
        Expr::Mul(a, b) => Ok(Literal::Num({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x * y)
        })),
        Expr::Div(a, b) => Ok(Literal::Num({
            let left = eval(a, vars, funcs)?;
            let right = eval(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x / y)
        })),
        Expr::Var(name) => {
            if let Some((_, value)) = vars.iter().rev().find(|(var, _)| var == name) {
                Ok(*value)
            } else {
                Err(format!("Cannot find variable `{}` in scope", name))
            }
        }
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((name.clone(), rhs));
            let output = eval(then, vars, funcs);
            vars.pop();
            output
        }
        Expr::Call(name, args) => {
            if let Some((_, arg_names, body)) = funcs
                .iter()
                .rev()
                .find(|(var, _, _)| var == &name)
                .copied()
            {
                if arg_names.len() == args.len() {
                    let mut args = args
                        .iter()
                        .map(|arg| eval(arg, vars, funcs))
                        .zip(arg_names.iter())
                        .map(|(val, name)| Ok((name.clone(), val?)))
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
            funcs.push((&name, &args, &body));
            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        }
    }
}

/// Evaluates souce string using [`parser()`]
pub fn eval_source(src: String) -> Result<Literal, Vec<String>> {
    match parser().then_ignore(end()).parse_recovery_verbose(src) {
        (Some(ast), _) => match eval(&ast, &mut Vec::new(), &mut Vec::new()) {
            Ok(output) => Ok(output),
            Err(eval_err) => Err(vec![format!("Evaluation error: {:?}", eval_err)]),
        },
        (None, parse_errs) => Err(parse_errs
            .into_iter()
            .map(|e| format!("Parse error: {:?}", e))
            .collect()),
    }
}
