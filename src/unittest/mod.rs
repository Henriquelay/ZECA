#[cfg(test)]
use chumsky::{prelude::end, Parser};

mod test_util;

#[test]
fn comment() {
    let comment_parser = crate::parser::comment;

    let ok_comments = [
        vec![
            "// this is a comment\n",
            "//\n",
            "// \n",
            "///\n",
            "// this is a comment // this still is\n",
            "// this is a comment\n// two in a row!\n",
        ],
        vec![
            "/* */",
            "/* a block comment */",
            "/* anoter block comment */",
            "/** a double block comment **/",
            "/* should still be valid **/",
            "/*so should this*/",
            "/**/",
            "/* this is
            multi line */",
            "/* /* */",
            "/** this is
            // multi line double **/",
            //TODO     "/* nested block comments
            //     /*
            //         Should be fine
            //      */
            // */",
        ],
        vec!["// both types of comments,\n/*but only one str*/"],
    ];

    test_util::ok(
        |s| {
            comment_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s)
        },
        ok_comments.iter().flatten(),
    );

    let bad_comments = [
        vec![
            "not a comment",
            "something before // comment ",
            "/incomplete",
            "/incomplete/",
            // "", // Didn't handle empty comment very well
            " ",
        ],
        vec![
            "/*/",
            "/* /",
            "/ */",
            "/*",
            "*/",
            "something before /* comment */",
            "/* comment */ something after",
        ],
    ];

    test_util::err(
        |s| {
            comment_parser()
                .then_ignore(end())
                .parse_recovery_verbose(*s)
        },
        bad_comments.iter().flatten(),
    );
}

#[test]
pub fn bool() {
    let bool_parser = crate::parser::boolean;

    let ok_bools = vec!["false", "true"];
    test_util::ok(
        |s| {
            bool_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        ok_bools.iter(),
    );

    let bad_bools = [
        "flase", "treu", " ", "t", "r", "u", "e", "f", "a", "l", "s", "e",
    ];
    test_util::err(
        |s| {
            bool_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        bad_bools.iter(),
    );
}

#[test]
pub fn int() {
    let int_parser = crate::parser::integer;

    let ok_ints = vec![
        "0",
        "123132123",
        // "123i32", "123usize", "123u32", "0usize", // "-1", // ?
        // "1isize", "2usize",
        // "123_u32",
        // "0xff",
        // "0xff_u8",
        // "0o70",
        // "0o70_i16",
        // "0b1111_1111_1001_0000",
        // "0b1111_1111_1001_0000i64",
        // "0b________1",
        // "0usize",
    ];
    test_util::ok(
        |s| {
            int_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        ok_ints.iter(),
    );

    let bad_ints = [
        // "0.1", // Should be a bad integer as soon as the parsers are separated
        "-888555", // Negative number are a composition of Neg(Number(Integer(x)))
        "0,1",
        "0invalidSuffix",
        "123AFB43",
        "0b_",
        "0b____",
    ];

    test_util::err(
        |s| {
            int_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        bad_ints.iter(),
    );
}

#[test]
pub fn float() {
    let float_parser = crate::parser::float;

    let ok_floats = vec![
        "2.0",
        "2.",
        // "12E+99",
        // "12.01E+99",
        // Type system
        // "123.0f64", "0.1f64", "0.1f32", "5f32",
        // "12E+99_f64",
        // "12.01E+99_f64",
    ];

    test_util::ok(
        |s| {
            float_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        ok_floats.iter(),
    );

    let bad_floats = vec![
        "2", "2f16", "2f8", "2.f64", // "2.E+99"
    ];

    test_util::err(
        |s| {
            float_parser().then_ignore(end()).parse_recovery_verbose(*s) // ???
        },
        bad_floats.iter(),
    );
}

#[test]
pub fn identifiers() {
    let identifier_parser = crate::parser::identifier;
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

    test_util::ok(
        |s| {
            identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        ok_identifiers.iter(),
    );

    let bad_identifiers = ["", " "];

    test_util::err(
        |s| {
            identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        bad_identifiers.iter(),
    );
}

#[test]
#[ignore]
// Ignored 🌹
pub fn raw_identifiers() {
    let identifier_parser = crate::parser::identifier;

    let ok_identifiers = [
        "r#x#",
        "r#variable#",
        "r#data#",
        "r#TEST#",
        "r#foo#",
        "r#_identifier#",
        "r#_#",
        // FIXME update to unicode XID
        // "Москва",
        // "東京",
        // "💯",
        // "r#true",
    ];

    test_util::ok(
        |s| {
            identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        ok_identifiers.iter(),
    );

    let bad_identifiers = ["r##", "r# #"];

    test_util::err(
        |s| {
            identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        bad_identifiers.iter(),
    );
}

/*
#[test]
fn symbols() {
    let symbols = crate::parser::symbols;
    let ok_punctuations = [
        "+", "-", "*", "/", "%", "^", "!", "&", "|", "&&", "||", "<<", ">>", "+=", "-=", "*=",
        "/=", "%=", "^=", "&=", "|=", "<<=", ">>=", "=", "==", "!=", ">", "<", ">=", "<=", "@",
        // "_",
        ".", "..", "...", "..=", ",", ";", ":", "::", "->", "=>", "#", "$", "?",
    ];

    test_util::ok(
        |s| {
            symbols()
                .then_ignore(end())
                .parse_recovery_verbose(*s)
        },
        ok_punctuations.iter(),
    );

    let bad_punctuations = ["", " ", "sla", ":)"];

    test_util::err(
        |s| {
            symbols()
                .then_ignore(end())
                .parse_recovery_verbose(*s)
        },
        bad_punctuations.iter(),
    );
}
*/

/*
#[ignore]
#[test]
pub fn keywords() {
    let keyword_parser = crate::parser::reserved;

    let ok_keywords = vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
        "Self", "static", "struct", "super", "trait", "type", "unsafe", "use", "where", "while",
    ];
    let ok_weak_keywords = vec!["macro_rules", "union", "'static"];
    let ok_test_suites = [ok_keywords, ok_weak_keywords];

    for ok_suite in ok_test_suites {
        test_util::ok(
            |s| {
                keyword_parser
                    .then_ignore(end())
                    .parse_recovery_verbose(*s)
            },
            ok_suite.iter(),
        );
    }

    let bad_keywords = vec!["something", "CONTINUE", "Continue", "CoNtInUe"];
    let bad_weak_keywords = vec!["Yield", "YIELD", "YiElD"];
    let bad_test_suites = [bad_keywords, bad_weak_keywords];

    for bad_suite in bad_test_suites {
        test_util::err(
            |s| {
                keyword_parser
                    .repeated()
                    .then_ignore(end())
                    .parse_recovery_verbose(*s)
            },
            bad_suite.iter(),
        );
    }
}
*/

/*
// Reserved for future use. These cause the thread to panic and therefore can only be tested one by one
#[test]
#[should_panic(expected = r#"Keyword "abstract" is not implemented. Panicking."#)]
fn reserved_for_future_use_abstract() {
    crate::parser::reserved.parse("abstract");
}

#[test]
#[should_panic(expected = r#"Keyword "become" is not implemented. Panicking."#)]
fn reserved_for_future_use_become() {
    crate::parser::reserved.parse("become");
}

#[test]
#[should_panic(expected = r#"Keyword "box" is not implemented. Panicking."#)]
fn reserved_for_future_use_box() {
    crate::parser::reserved.parse("box");
}

#[test]
#[should_panic(expected = r#"Keyword "do" is not implemented. Panicking."#)]
fn reserved_for_future_use_do() {
    crate::parser::reserved.parse("do");
}

#[test]
#[should_panic(expected = r#"Keyword "final" is not implemented. Panicking."#)]
fn reserved_for_future_use_final() {
    crate::parser::reserved.parse("final");
}

#[test]
#[should_panic(expected = r#"Keyword "macro" is not implemented. Panicking."#)]
fn reserved_for_future_use_macro() {
    crate::parser::reserved.parse("macro");
}

#[test]
#[should_panic(expected = r#"Keyword "override" is not implemented. Panicking."#)]
fn reserved_for_future_use_override() {
    crate::parser::reserved.parse("override");
}

#[test]
#[should_panic(expected = r#"Keyword "priv" is not implemented. Panicking."#)]
fn reserved_for_future_use_priv() {
    crate::parser::reserved.parse("priv");
}

#[test]
#[should_panic(expected = r#"Keyword "typeof" is not implemented. Panicking."#)]
fn reserved_for_future_use_typeof() {
    crate::parser::reserved.parse("typeof");
}

#[test]
#[should_panic(expected = r#"Keyword "unsized" is not implemented. Panicking."#)]
fn reserved_for_future_use_unsized() {
    crate::parser::reserved.parse("unsized");
}

#[test]
#[should_panic(expected = r#"Keyword "virtual" is not implemented. Panicking."#)]
fn reserved_for_future_use_virtual() {
    crate::parser::reserved.parse("virtual");
}

#[test]
#[should_panic(expected = r#"Keyword "yield" is not implemented. Panicking."#)]
fn reserved_for_future_use_yield() {
    crate::parser::reserved.parse("yield");
}
*/
