use super::log;

#[test]
pub fn itentifiers() {
    let identifier_parser = crate::grammar::IdentifierParser::new();
    let ok_identifiers = [
        "x",
        "variable",
        "data",
        "TEST",
        "foo",
        "_identifier",
        "_",
        // "Москва",
        // "東京",
        // FIXME update to unicode XID
        // "💯",
        // "r#true",
    ];
    for id in ok_identifiers {
        log::debug!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_ok());
    }

    let bad_identifiers = ["", " "];
    for id in bad_identifiers {
        log::debug!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_err());
    }

    // TODO
    let not_identifiers = ["fn", "'static", "struct"];
}
    

    
