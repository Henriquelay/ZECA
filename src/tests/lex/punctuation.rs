#[test]
fn test() {
    let punctuation_parser = crate::grammar::PunctuationParser::new();
    let ok_punctuations = [
        "+", "-", "*", "/", "%", "^", "!", "&", "|", "&&", "||", "<<", ">>", "+=", "-=", "*=",
        "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", "=", "==", "!=", ">", "<", ">=", "<=", "@",
        // "_",
        ".", "..", "...", "..=", ",", ";", ":", "::", "->", "=>", "#", "$", "?",
    ];
    for id in ok_punctuations {
        println!("Testing {}", id);
        assert!(punctuation_parser.parse(id).is_ok());
    }

    let bad_punctuations = ["", " ", "sla", ":)"];
    for id in bad_punctuations {
        println!("Testing {}", id);
        assert!(punctuation_parser.parse(id).is_err());
    }
}
