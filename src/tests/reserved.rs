use super::log;

#[test]
pub fn keywords() {
    let keyword_parser = crate::grammar::ReservedParser::new();

    let ok_keywords = vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while",
    ];
    let ok_weak_keywords = vec!["macro_rules", "union", "'static"];
    let ok_test_suites = vec![ok_keywords, ok_weak_keywords];

    for suite in ok_test_suites {
        for kw in suite {
            log::debug!("Testing {}", kw);
            assert!(keyword_parser.parse(kw).is_ok());
        }
    }

    let bad_keywords = vec!["something", "CONTINUE", "Continue", "CoNtInUe"];
    let bad_weak_keywords = vec!["Yield", "YIELD", "YiElD"];
    let bad_test_suites = vec![bad_keywords, bad_weak_keywords];

    for swuite in bad_test_suites {
        for kw in swuite {
            log::debug!("Testing {}", kw);
            assert!(keyword_parser.parse(kw).is_err());
        }
    }
}

// Reserved for future use. These causes the thread to panic and therefore can only be tested one by one
#[test]
#[should_panic(expected = r#"Keyword "abstract" is not implemented. Panicking."#)]
fn reserved_for_future_use_abstract() {
    crate::grammar::ReservedParser::new().parse("abstract");
}

#[test]
#[should_panic(expected = r#"Keyword "become" is not implemented. Panicking."#)]
fn reserved_for_future_use_become() {
    crate::grammar::ReservedParser::new().parse("become");
}

#[test]
#[should_panic(expected = r#"Keyword "box" is not implemented. Panicking."#)]
fn reserved_for_future_use_box() {
    crate::grammar::ReservedParser::new().parse("box");
}

#[test]
#[should_panic(expected = r#"Keyword "do" is not implemented. Panicking."#)]
fn reserved_for_future_use_do() {
    crate::grammar::ReservedParser::new().parse("do");
}

#[test]
#[should_panic(expected = r#"Keyword "final" is not implemented. Panicking."#)]
fn reserved_for_future_use_final() {
    crate::grammar::ReservedParser::new().parse("final");
}

#[test]
#[should_panic(expected = r#"Keyword "macro" is not implemented. Panicking."#)]
fn reserved_for_future_use_macro() {
    crate::grammar::ReservedParser::new().parse("macro");
}

#[test]
#[should_panic(expected = r#"Keyword "override" is not implemented. Panicking."#)]
fn reserved_for_future_use_override() {
    crate::grammar::ReservedParser::new().parse("override");
}

#[test]
#[should_panic(expected = r#"Keyword "priv" is not implemented. Panicking."#)]
fn reserved_for_future_use_priv() {
    crate::grammar::ReservedParser::new().parse("priv");
}

#[test]
#[should_panic(expected = r#"Keyword "typeof" is not implemented. Panicking."#)]
fn reserved_for_future_use_typeof() {
    crate::grammar::ReservedParser::new().parse("typeof");
}

#[test]
#[should_panic(expected = r#"Keyword "unsized" is not implemented. Panicking."#)]
fn reserved_for_future_use_unsized() {
    crate::grammar::ReservedParser::new().parse("unsized");
}

#[test]
#[should_panic(expected = r#"Keyword "virtual" is not implemented. Panicking."#)]
fn reserved_for_future_use_virtual() {
    crate::grammar::ReservedParser::new().parse("virtual");
}

#[test]
#[should_panic(expected = r#"Keyword "yield" is not implemented. Panicking."#)]
fn reserved_for_future_use_yield() {
    crate::grammar::ReservedParser::new().parse("yield");
}
