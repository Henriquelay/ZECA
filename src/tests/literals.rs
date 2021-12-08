use super::log;

#[test]
pub fn characters() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_characters = [r#"'c'"#, r#"' '"#, r#"'\\'"#, r#"'\n'"#, r#"'\0'"#];
    for c in ok_characters {
        log::debug!("Testing {}", c);
        assert!(literal_parser.parse(c).is_ok());
    }

    let bad_characters = ["''", "'teste'"];
    for c in bad_characters {
        log::debug!("Testing {}", c);
        assert!(literal_parser.parse(c).is_err());
    }
}

#[test]
pub fn byte_characters() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_characters = [r#"b'c'"#, r#"b' '"#, r#"b'\\'"#, r#"b'\n'"#, r#"b'\0'"#];
    for c in ok_characters {
        log::debug!("Testing {}", c);
        assert!(literal_parser.parse(c).is_ok());
    }

    let bad_characters = ["b''", "b'teste'"];
    for c in bad_characters {
        log::debug!("Testing {}", c);
        assert!(literal_parser.parse(c).is_err());
    }
}

#[test]
pub fn str() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = [
        r#""uma stringue""#,
        r#""""#,
        r#""\0 ocm escape""#,
        r#""\\ \" teste teste fdc \n
        aaaaa""#,
        r##""\\ \" teste #t \# #este fdc \n
        aaaaa""##,
    ];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = [r#""'"#, r#""sem fechar"#];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}

#[test]
pub fn byte_str() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = [
        r#"b"uma stringue""#,
        r#"b"""#,
        r#"b"\0 ocm escape""#,
        r#"b"\\ \" teste teste fdc \n
        aaaaa""#,
        r##"b"\\ \" teste #t \# #este fdc \n
        aaaaa""##,
    ];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = [r#"b"'"#, r#"b"sem fechar"#];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}

#[test]
pub fn raw_str() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = [
        r###"r#"uma stringue vc parsearia agui"#"###,
        // r###"r#""#"###,
        // r###"r##"\0 ocm escape"##"###,
        // r###"r#"\\ " teste teste fdc
        // aaaaa"#"###,
        // r####"r##"\\ \" teste #t \# #este fdc \n
        // aaaaa"##"####,
    ];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = [r###"r"'"###, r###"r"sem fechar""###];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}

#[test]
pub fn raw_byte_str() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = [
        r###"br#"uma stringue vc parsearia agui"#"###,
        // r###"br#""#"###,
        // r###"br##"\0 ocm escape"##"###,
        // r###"br#"\\ " teste teste fdc
        // aaaaa"#"###,
        // r####"br##"\\ \" teste #t \# #este fdc \n
        // aaaaa"##"####,
    ];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = [r###"br"'"###, r###"br"sem fechar""###];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}

#[test]
pub fn bool() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = ["false", "true"];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = ["flase", "treu", ""];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}

#[test]
pub fn integer() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_strs = [
        "0",
        "-1", // ?
        "1",
        "2",
        "123",
        "123i32",
        "123u32",
        "123_u32",
        "0xff",
        "0xff_u8",
        "0o70",
        "0o70_i16",
        "0b1111_1111_1001_0000",
        "0b1111_1111_1001_0000i64",
        "0b________1",
        "0usize",
    ];
    for s in ok_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_ok());
    }

    let bad_strs = ["0.1", "0,1", "0invalidSuffix", "123AFB43", "0b_", "0b____"];
    for s in bad_strs {
        log::debug!("Testing {}", s);
        assert!(literal_parser.parse(s).is_err());
    }
}
