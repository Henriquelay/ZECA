use super::log;

#[test]
pub fn characters() {
    let literal_parser = crate::grammar::LiteralParser::new();

    let ok_characters = [
        r#"'c'"#,
        r#"' '"#,
        r#"'\\'"#,
        r#"'\n'"#,
        r#"'\0'"#,
        ];
    for id in ok_characters {
        log::debug!("Testing {}", id);
        assert!(literal_parser.parse(id).is_ok());
    }

    let bad_characters = ["''", "'teste'"];
    for id in bad_characters {
        log::debug!("Testing {}", id);
        assert!(literal_parser.parse(id).is_err());
    }

    // TODO
    let not_characters = ["fn", "'static", "struct"];
}
