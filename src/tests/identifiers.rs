use super::log;

#[test]
pub fn itentifiers() {
    let identifier_parser = crate::grammar::IdentifierParser::new();
    let ok_identifiers = vec![
        "x",
        "variable",
        "data",
        "TEST",
        "foo",
        "_identifier",
        "Москва",
        "東京",
        "_",
        // FIXME
        // "💯",
        // "r#true",
    ];

    let bad_identifiers = vec!["", " "];

    let not_identifiers = vec!["fn", "'static", "struct"];

    for id in ok_identifiers {
        log::debug!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_ok());
    }
    
    for id in bad_identifiers {
        log::debug!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_err());
    }
}
