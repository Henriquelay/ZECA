#[test]
pub fn keywords() {
    let program_parser = crate::grammar::ProgramParser::new();
    assert!(program_parser.parse("as").is_ok());
    assert!(program_parser.parse("break").is_ok());
    assert!(program_parser.parse("const").is_ok());
    assert!(crate::grammar::ProgramParser::new()
        .parse("continue")
        .is_ok());
    assert!(program_parser.parse("crate").is_ok());
    assert!(program_parser.parse("else").is_ok());
    assert!(program_parser.parse("enum").is_ok());
    assert!(program_parser.parse("extern").is_ok());
    assert!(program_parser.parse("false").is_ok());
    assert!(program_parser.parse("fn").is_ok());
    assert!(program_parser.parse("for").is_ok());
    assert!(program_parser.parse("if").is_ok());
    assert!(program_parser.parse("impl").is_ok());
    assert!(program_parser.parse("in").is_ok());
    assert!(program_parser.parse("let").is_ok());
    assert!(program_parser.parse("loop").is_ok());
    assert!(program_parser.parse("match").is_ok());
    assert!(program_parser.parse("mod").is_ok());
    assert!(program_parser.parse("move").is_ok());
    assert!(program_parser.parse("mut").is_ok());
    assert!(program_parser.parse("pub").is_ok());
    assert!(program_parser.parse("ref").is_ok());
    assert!(program_parser.parse("return").is_ok());
    assert!(program_parser.parse("self").is_ok());
    assert!(program_parser.parse("Self").is_ok());
    assert!(program_parser.parse("static").is_ok());
    assert!(program_parser.parse("struct").is_ok());
    assert!(program_parser.parse("super").is_ok());
    assert!(program_parser.parse("trait").is_ok());
    assert!(program_parser.parse("true").is_ok());
    assert!(program_parser.parse("type").is_ok());
    assert!(program_parser.parse("unsafe").is_ok());
    assert!(program_parser.parse("use").is_ok());
    assert!(program_parser.parse("where").is_ok());
    assert!(program_parser.parse("while").is_ok());

    assert!(program_parser.parse("something").is_err());
    assert!(program_parser.parse("CONTINUE").is_err());
    assert!(program_parser.parse("Continue").is_err());
    assert!(program_parser.parse("CoNtInUe").is_err());

    assert!(program_parser.parse("Yield").is_err());
    assert!(program_parser.parse("YIELD").is_err());
    assert!(program_parser.parse("YiElD").is_err());
}

// Reserved for future use

#[test]
#[should_panic]
fn reserved_for_future_use_abstract() {
    crate::grammar::ProgramParser::new().parse("abstract");
}

#[test]
#[should_panic]
fn reserved_for_future_use_become() {
    crate::grammar::ProgramParser::new().parse("become");
}

#[test]
#[should_panic]
fn reserved_for_future_use_box() {
    crate::grammar::ProgramParser::new().parse("box");
}

#[test]
#[should_panic]
fn reserved_for_future_use_do() {
    crate::grammar::ProgramParser::new().parse("do");
}

#[test]
#[should_panic]
fn reserved_for_future_use_final() {
    crate::grammar::ProgramParser::new().parse("final");
}

#[test]
#[should_panic]
fn reserved_for_future_use_macro() {
    crate::grammar::ProgramParser::new().parse("macro");
}

#[test]
#[should_panic]
fn reserved_for_future_use_override() {
    crate::grammar::ProgramParser::new().parse("override");
}

#[test]
#[should_panic]
fn reserved_for_future_use_priv() {
    crate::grammar::ProgramParser::new().parse("priv");
}

#[test]
#[should_panic]
fn reserved_for_future_use_typeof() {
    crate::grammar::ProgramParser::new().parse("typeof");
}

#[test]
#[should_panic]
fn reserved_for_future_use_unsized() {
    crate::grammar::ProgramParser::new().parse("unsized");
}

#[test]
#[should_panic]
fn reserved_for_future_use_virtual() {
    crate::grammar::ProgramParser::new().parse("virtual");
}

#[test]
#[should_panic]
fn reserved_for_future_use_yield() {
    crate::grammar::ProgramParser::new().parse("yield");
}
