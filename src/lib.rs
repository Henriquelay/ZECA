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

use std::collections::HashMap;

use chumsky::{prelude::end, text::TextParser, Parser};
use parser::ast::*;

macro_rules! for_every_number_Value {
    ($expr:expr, $clj:expr) => {
        match $expr {
            (Literal::Num(n), Literal::Num(o)) => match (n, o) {
                (Number::Integer(x), Number::Integer(y)) => $clj(x, y),
                // (Number::UInteger(x), Number::UInteger(y)) => $clj(x, y),
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
                // (Number::UInteger(x), Number::UInteger(y)) => Number::UInteger($clj(x, y)),
                (Number::Float(x), Number::Float(y)) => Number::Float($clj(x, y)),
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    };
}

/// Evaluates return value
fn eval_expr<'a>(
    expr: &'a Expr,
    vars: &mut Vec<HashMap<String, Vec<Literal>>>,
    funcs: &HashMap<String, &Function>,
) -> Result<Literal, String> {
    match expr {
        Expr::Literal(x) => Ok(x.clone()),
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
            _ => Err("Cannot apply negation".to_string()),
        },
        Expr::And(a, b) => match (eval_expr(a, vars, funcs)?, eval_expr(b, vars, funcs)?) {
            (Literal::Num(x), Literal::Num(y)) => Ok(Literal::Bool(
                x > Number::Integer(1) && y > Number::Integer(1),
            )),
            (Literal::Bool(x), Literal::Bool(y)) => Ok(Literal::Bool(x && y)),
            _ => Err("Cannot apply AND".to_string()),
        },
        Expr::Or(a, b) => match (eval_expr(a, vars, funcs)?, eval_expr(b, vars, funcs)?) {
            (Literal::Num(x), Literal::Num(y)) => Ok(Literal::Bool(
                x > Number::Integer(1) || y > Number::Integer(1),
            )),
            (Literal::Bool(x), Literal::Bool(y)) => Ok(Literal::Bool(x || y)),
            _ => Err("Cannot apply OR".to_string()),
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
        Expr::Var { name, index } => {
            // Searches the variable on variables symbol table that matches name with invoked variable
            let mut retval = None;
            for scope_vars in vars.iter().rev() {
                if let Some(var_vec) = scope_vars.get(name) {
                    if let Some(var) = var_vec.iter().last() {
                        retval = Some(Ok(var.to_owned()));
                        break;
                    } else {
                        retval = Some(Err(format!("Cannot find variable `{}` in scope", name)));
                    }
                } else {
                    retval = Some(Err(format!("Cannot find variable `{}`", name)));
                }
            }

            // If index is some, return value at index, if value is array
            if let &Expr::Literal(Literal::Num(Number::Integer(index_number))) = index.as_ref() {
                if index_number > 0 {
                    if let Literal::Array(array) = retval.unwrap().unwrap() {
                        retval = Some(Ok(array[index_number as usize].clone()))
                    } else {
                        retval = Some(Err("Cannot index non-array".to_string()))
                    }
                }
            } else {
                retval = Some(Err("Cannot index with valu less than zero".to_string()));
            }
            retval.unwrap()
        }
        Expr::Array(array) => {
            let mut retval = Vec::new();
            for expr in array {
                retval.push(eval_expr(expr, vars, funcs)?);
            }
            Ok(Literal::Array(retval))
        }
        Expr::Call(name, call_args) => {
            // Retrieve the callee signature
            if let Some(function) = funcs.get(name) {
                if function.args.len() == call_args.len() {
                    // Combine passed args to argument name as `(name, Value)`, similar to variables
                    let args = function.args.iter().zip(
                        call_args
                            .iter()
                            .map(|arg| eval_expr(arg, vars, funcs).unwrap()),
                    );

                    let mut new_scope = HashMap::new();

                    // Include passed variables to the scope inside the function
                    for arg in args {
                        new_scope.insert(arg.0.to_owned(), vec![arg.1]);
                    }

                    vars.push(new_scope);

                    // Evaluated the function result
                    let output = eval(&function.body, vars, funcs, false);
                    // Remove passed vars from scope, as the scope is outside the function now

                    vars.pop();

                    output
                } else {
                    Err(format!(
                        "Wrong number of arguments for function `{}`: expected {}, found {}",
                        name,
                        function.args.len(),
                        call_args.len(),
                    ))
                }
            } else {
                Err(format!("Cannot find function `{}`.", name))
            }
        }
    }
}

/// Evaluates return value for block
fn eval<'a>(
    blk: &'a Block,
    vars: &mut Vec<HashMap<String, Vec<Literal>>>,
    funcs: &HashMap<String, &Function>,
    is_loop: bool,
) -> Result<Literal, String> {
    let mut last_statement = None;
    for statement in blk.0.clone() {
        last_statement = Some(match statement {
            Statement::Expr(expr) => Ok(eval_expr(&expr, vars, funcs)?),
            Statement::Block(blk) => Ok(eval(&blk, vars, funcs, false)?),
            Statement::Break => Ok(if is_loop {
                Literal::Break // Break special value to signal last eval broke
            } else {
                Literal::Null // Break outside loops are no-op
            }),
            Statement::Loop(r#loop) => match *r#loop {
                Loop(blk) => Ok(loop {
                    if let Literal::Break = eval(&blk, vars, funcs, true)? {
                        break Literal::Null;
                    }
                }),
            },
            Statement::Item(_item) => todo!(),
            Statement::Conditional {
                r#if,
                r#then,
                r#else,
            } => {
                if let Literal::Bool(cond) = eval_expr(&r#if, vars, funcs)? {
                    if cond {
                        Ok(eval(&r#then, vars, funcs, is_loop)?)
                    } else if let Some(r#else) = r#else {
                        Ok(eval(&r#else, vars, funcs, is_loop)?)
                    } else {
                        Ok(Literal::Null)
                    }
                } else {
                    Err("Conditional's condition is not a boolean expression".to_string())
                }
            }
            Statement::Let { lvalue, rvalue } => {
                // Evaluates RHS first
                let rvalue = eval_expr(&rvalue, vars, funcs)?;
                // Pushes name into variable symbol table
                // TODO shadones (se já tiver no vec, só coloca no final)
                vars.last_mut()
                    .unwrap()
                    .insert(lvalue.clone(), vec![rvalue.clone()]);
                Ok(rvalue)
            }
            Statement::Assign { lvalue, rvalue } => {
                let new_value = eval_expr(&rvalue, vars, funcs)?;
                vars.last_mut()
                    .unwrap()
                    // TODO shadones
                    .insert(lvalue.clone(), vec![new_value.clone()]);
                Ok(new_value)
            }
            Statement::Null => Ok(Literal::Null),
        });
        match last_statement {
            Some(Ok(Literal::Break)) => break,
            _ => continue,
        }
    }
    Ok(last_statement.unwrap().unwrap())
}

/// Evaluates source string using [`parser!()`]
pub fn eval_source(src: String) -> Result<Literal, Vec<String>> {
    match parser!().then_ignore(end()).parse_recovery_verbose(src) {
        // Extract `main()` function
        (Some(ast), _) => {
            // Register all function items
            let mut funcs: HashMap<String, &Function> = HashMap::new();
            for item in ast.iter() {
                match item {
                    Item::Function(f) => funcs.insert(f.name.clone(), &Box::new(f)),
                };
            }
            // Searching for function called `main`
            if let Some(Item::Function(main)) = ast.iter().find(|&item| match item {
                Item::Function(Function {
                    name,
                    args: _,
                    body: _,
                }) => name == "main",
            }) {
                // Evaluate `main(){ }
                match eval(&main.body, &mut vec![HashMap::new()], &funcs, false) {
                    Ok(output) => Ok(output),
                    Err(eval_err) => Err(vec![format!("Evaluation error: {:?}", eval_err)]),
                }
            } else {
                Err(vec![
                    "Syntax error: No function named `main` in top-level items. Can't continue."
                        .to_string(),
                ])
            }
        }
        (None, parse_errs) => Err(parse_errs
            .into_iter()
            .map(|e| format!("Parse error: {:?}", e))
            .collect()),
    }
}
