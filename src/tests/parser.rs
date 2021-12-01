#[test]
pub fn keywords() {
    let program_parser = crate::grammar::ProgramParser::new();
    let ok_keywords = vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];

    let bad_keywords = vec!["something", "CONTINUE", "Continue", "CoNtInUe"];

    let bad_weak_keywords = vec!["Yield", "YIELD", "YiElD"];

    for kw in ok_keywords {
        assert!(program_parser.parse(kw).is_ok());
    }

    for kw in bad_keywords {
        assert!(program_parser.parse(kw).is_err());
    }

    for kw in bad_weak_keywords {
        assert!(program_parser.parse(kw).is_err());
    }
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
