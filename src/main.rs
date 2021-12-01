#![allow(unused)]
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(grammar);

pub mod ast;

#[cfg(not(test))]
fn main() {}

#[test]
fn luthor() {
    assert!(grammar::ProgramParser::new().parse("as").is_ok());
    assert!(grammar::ProgramParser::new().parse("break").is_ok());
    assert!(grammar::ProgramParser::new().parse("const").is_ok());
    assert!(grammar::ProgramParser::new().parse("continue").is_ok());
    assert!(grammar::ProgramParser::new().parse("crate").is_ok());
    assert!(grammar::ProgramParser::new().parse("else").is_ok());
    assert!(grammar::ProgramParser::new().parse("enum").is_ok());
    assert!(grammar::ProgramParser::new().parse("extern").is_ok());
    assert!(grammar::ProgramParser::new().parse("false").is_ok());
    assert!(grammar::ProgramParser::new().parse("fn").is_ok());
    assert!(grammar::ProgramParser::new().parse("for").is_ok());
    assert!(grammar::ProgramParser::new().parse("if").is_ok());
    assert!(grammar::ProgramParser::new().parse("impl").is_ok());
    assert!(grammar::ProgramParser::new().parse("in").is_ok());
    assert!(grammar::ProgramParser::new().parse("let").is_ok());
    assert!(grammar::ProgramParser::new().parse("loop").is_ok());
    assert!(grammar::ProgramParser::new().parse("match").is_ok());
    assert!(grammar::ProgramParser::new().parse("mod").is_ok());
    assert!(grammar::ProgramParser::new().parse("move").is_ok());
    assert!(grammar::ProgramParser::new().parse("mut").is_ok());
    assert!(grammar::ProgramParser::new().parse("pub").is_ok());
    assert!(grammar::ProgramParser::new().parse("ref").is_ok());
    assert!(grammar::ProgramParser::new().parse("return").is_ok());
    assert!(grammar::ProgramParser::new().parse("self").is_ok());
    assert!(grammar::ProgramParser::new().parse("Self").is_ok());
    assert!(grammar::ProgramParser::new().parse("static").is_ok());
    assert!(grammar::ProgramParser::new().parse("struct").is_ok());
    assert!(grammar::ProgramParser::new().parse("super").is_ok());
    assert!(grammar::ProgramParser::new().parse("trait").is_ok());
    assert!(grammar::ProgramParser::new().parse("true").is_ok());
    assert!(grammar::ProgramParser::new().parse("type").is_ok());
    assert!(grammar::ProgramParser::new().parse("unsafe").is_ok());
    assert!(grammar::ProgramParser::new().parse("use").is_ok());
    assert!(grammar::ProgramParser::new().parse("where").is_ok());
    assert!(grammar::ProgramParser::new().parse("while").is_ok());
}
