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
use parser::{ast::Expr, parser};
use chumsky::{Parser, prelude::end};

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

/// Evaluates souce string using [`parser()`]
pub fn eval_source(src: String) -> Result<f64, Vec<String>> {
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
