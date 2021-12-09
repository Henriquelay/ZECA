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
        // FIXME update to unicode XID
        // "Москва",
        // "東京",
        // "💯",
        // "r#true",
    ];
    for id in ok_identifiers {
        println!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_ok());
    }

    let bad_identifiers = ["", " "];
    for id in bad_identifiers {
        println!("Testing {}", id);
        assert!(identifier_parser.parse(id).is_err());
    }

    // TODO
    let not_identifiers = ["fn", "'static", "struct"];
}
    

    
