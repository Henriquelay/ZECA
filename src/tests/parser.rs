#[test]
pub fn keywords() {
    assert!(crate::grammar::ProgramParser::new().parse("as").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("break").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("const").is_ok());
    assert!(crate::grammar::ProgramParser::new()
        .parse("continue")
        .is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("crate").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("else").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("enum").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("extern").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("false").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("fn").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("for").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("if").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("impl").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("in").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("let").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("loop").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("match").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("mod").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("move").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("mut").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("pub").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("ref").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("return").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("self").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("Self").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("static").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("struct").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("super").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("trait").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("true").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("type").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("unsafe").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("use").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("where").is_ok());
    assert!(crate::grammar::ProgramParser::new().parse("while").is_ok());

    assert!(crate::grammar::ProgramParser::new()
        .parse("something")
        .is_err());
    assert!(crate::grammar::ProgramParser::new()
        .parse("CONTINUE")
        .is_err());
    assert!(crate::grammar::ProgramParser::new()
        .parse("Continue")
        .is_err());
    assert!(crate::grammar::ProgramParser::new()
        .parse("CoNtInUe")
        .is_err());

    assert!(crate::grammar::ProgramParser::new()
        .parse("Yield")
        .is_err());
    assert!(crate::grammar::ProgramParser::new()
        .parse("YIELD")
        .is_err());
    assert!(crate::grammar::ProgramParser::new()
        .parse("YiElD")
        .is_err());
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
