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

    // TODO
    let not_characters = ["fn", "'static", "struct"];
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

    // TODO
    let not_characters = ["fn", "'static", "struct"];
}
