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

/// Evaluates return value.
fn eval_expr<'a>(
    expr: &'a Expr,
    // TODO(optimization) replace both symbol list with actual symbol tables, not lists
    vars: &mut Vec<(String, Literal)>,
    funcs: &Vec<Function>,
) -> Result<Literal, String> {
    match expr {
        Expr::Literal(x) => Ok(*x),
        Expr::Lt(a, b) => Ok(Literal::Bool({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x < y)
        })),
        Expr::Gt(a, b) => Ok(Literal::Bool({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x > y)
        })),
        Expr::Eq(a, b) => Ok(Literal::Bool({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value!((left, right), |x, y| x == y)
        })),
        Expr::Neg(a) => match eval_expr(a, vars, funcs)? {
            Literal::Num(x) => Ok(Literal::Num(-x)),
            Literal::Bool(x) => Ok(Literal::Bool(!x)),
            Literal::Null => Ok(Literal::Null),
        },
        Expr::Add(a, b) => Ok(Literal::Num({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x + y)
        })),
        Expr::Sub(a, b) => Ok(Literal::Num({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x - y)
        })),
        Expr::Mul(a, b) => Ok(Literal::Num({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x * y)
        })),
        Expr::Div(a, b) => Ok(Literal::Num({
            let left = eval_expr(a, vars, funcs)?;
            let right = eval_expr(b, vars, funcs)?;
            for_every_number_Value_wrapped!((left, right), |x, y| x / y)
        })),
        Expr::Var(name) => {
            // Searches the variable on variables symbol table that matches name with invoked variable
            let search_var = |name| vars.iter().rev().find(|(var, _)| var == name);
            if let Some((_, value)) = search_var(name) {
                Ok(*value)
            } else {
                Err(format!("Cannot find variable `{}` in scope", name))
            }
        }
        Expr::Call(name, args) => {
            // Retrieve the callee signature
            if let Some(function) = funcs.iter().rev().find(|function| function.name == *name) {
                if function.args.len() == args.len() {
                    // Combine passed args to argument name as `(name, Value)`, similar to variables
                    let mut args = args
                        .iter()
                        .map(|arg| eval_expr(arg, vars, funcs))
                        .zip(function.args.iter())
                        .map(|(val, name)| Ok((name.clone(), val?)))
                        .collect::<Result<_, String>>()?;
                    // Include passed variables to the scope inside the function
                    vars.append(&mut args);
                    // Evaluated the function result
                    let output = eval(&function.body, vars, funcs);
                    // Remove passed vars from scope, as the score is outside the function now
                    vars.truncate(vars.len() - args.len());
                    output
                } else {
                    Err(format!(
                        "Wrong number of arguments for function `{}`: expected {}, found {}",
                        name,
                        function.args.len(),
                        args.len(),
                    ))
                }
            } else {
                Err(format!("Cannot find function `{}` in scope", name))
            }
        }
    }
}

/// Evaluates return value.
fn eval<'a>(
    blk: &'a Block,
    // TODO(optimization) replace both symbol list with actual symbol tables, not lists
    vars: &mut Vec<(String, Literal)>,
    funcs: &Vec<Function>,
) -> Result<Literal, String> {
    let mut last_statement = None;
    for statement in blk.0.clone() {
        last_statement = Some(match statement {
            Statement::Expr(expr) => eval_expr(&expr, vars, funcs)?,
            Statement::Item(item) => match item {
                _ => todo!(),
            },
            Statement::Let { name, rhs } => {
                // Evaluates RHS first
                let rhs = eval_expr(&rhs, vars, funcs)?;
                // Pushes name into variable symbol table
                vars.push((name.clone(), rhs));
                rhs
            }
            Statement::Null => Literal::Null,
        })
    }
    Ok(last_statement.unwrap())
}

/// Evaluates source string using [`parser()`].
pub fn eval_source(src: String) -> Result<Literal, Vec<String>> {
    match parser().then_ignore(end()).parse_recovery_verbose(src) {
        // Extract `main()` function
        (Some(ast), _) => {
            if let Some(Item::Function(main)) = ast.iter().find(|&item| match item {
                Item::Function(Function { name, args: _, body: _ }) => name == "main",
            }) {
                // Register all function items
                let mut funcs = ast.iter().map(|item| match item {
                    Item::Function(f) => f.to_owned(),
                    // _ => unreachable!(),
                }).collect();
                // Evaluate `main(){ }
                match eval(&main.body, &mut Vec::new(), &mut funcs) {
                    Ok(output) => Ok(output),
                    Err(eval_err) => Err(vec![format!("Evaluation error: {:?}", eval_err)]),
                }
            } else {
                Err(vec![
                    "Syntax error: No function named `main` in top-level items.".to_string(),
                ])
            }
        }
        (None, parse_errs) => Err(parse_errs
            .into_iter()
            .map(|e| format!("Parse error: {:?}", e))
            .collect()),
    }
}
