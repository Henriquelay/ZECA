#![allow(unused)]
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

pub mod ast;

#[cfg(not(test))]
fn main() {}


#[cfg(test)]
mod tests;
