#[cfg(test)]
use chumsky::{prelude::end, Parser};

mod test_util;

#[test]
fn comment() {
    test_util::tests(
        |s| {
            crate::parser::comment_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
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
                // "/* nested block comments
                //     /*
                //         Should be fine
                //      */
                //     even with something after
                // */",
            ],
            vec!["// both types of comments,\n/*but only one str*/"],
        ]
        .into_iter()
        .flatten(),
        vec![
            vec![
                "not a comment",
                "something before // comment ",
                "/incomplete",
                "/incomplete/",
                // FIXME Didn't handle empty comment very well (shouldn't be a comment)
                // "",
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
        ]
        .into_iter()
        .flatten(),
    );
}

#[test]
pub fn bool() {
    test_util::tests(
        |s| {
            crate::parser::boolean_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec!["false", "true"],
        vec![
            "flase", "treu", " ", "t", "r", "u", "e", "f", "a", "l", "s", "e",
        ],
    );
}

#[test]
pub fn int() {
    test_util::tests(
        |s| {
            crate::parser::integer_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
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
        ],
        vec![
            // "0.1", // Should be a bad integer as soon as the parsers are separated
            "-888555", // Negative number are a composition of Neg(Number(Integer(x)))
            "0,1",
            "0invalidSuffix",
            "123AFB43",
            "0b_",
            "0b____",
        ],
    );
}

#[test]
pub fn float() {
    test_util::tests(
        |s| {
            crate::parser::float_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "2.0",
            "2.",
            // ANCHOR[id=scientific-notation-test]
            // "12E+99",
            // "12.01E+99",
            // "123.0f64", "0.1f64", "0.1f32", "5f32",
            // "12E+99_f64",
            // "12.01E+99_f64",
        ],
        vec![
            "2", "2f16", "2f8", "2.f64", "2,6", // "2.E+99"
        ],
    );
}

#[test]
pub fn identifiers() {
    test_util::tests(
        |s| {
            crate::parser::identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "x",
            "variable",
            "data",
            "TEST",
            "foo",
            "_identifier",
            "_",
            // ANCHOR[id=unicode-tests]
            // "????????????",
            // "??????",
            // "????",
            // "r#true",
        ],
        vec!["", " "],
    );
}

#[test]
#[ignore] // ????
pub fn raw_identifiers() {
    test_util::tests(
        |s| {
            crate::parser::identifier_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "r#x#",
            "r#variable#",
            "r#data#",
            "r#TEST#",
            "r#foo#",
            "r#_identifier#",
            "r#_#",
            // "????????????",
            // "??????",
            // "????",
            // "r#true",
        ],
        vec!["r##", "r# #"],
    );
}

#[test]
pub fn string() {
    test_util::tests(
        |s| {
            crate::parser::string_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![r#""""#, r#""\""#, r#""uma string feliz :)""#],
        vec![
            r#"string triste :("#,
            r#"""#,
            r#""string sem fechar"#,
            r#"''"#,
            r#"'test'"#,
        ],
    );
}

#[test]
pub fn assign() {
    test_util::tests(
        |s| {
            crate::parser::assignment_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            r#"x = 10;"#,
            r#"x = -10;"#,
            r#"complicated_name = 0.1;"#,
            r#"a = true;"#,
            r#"a = "strings";"#,
            r#"a = [];"#,
            r#"a = [1];"#,
            r#"a = [1, 2, 3];"#,
            r#"a = another_variable;"#,
        ],
        vec![
            r#"let a = "#,
            r#"name with space = "#,
            r#"array[index] = "#, // TODO not yet implemented
        ],
    );
}

#[test]
pub fn expr() {
    test_util::tests(
        |s| {
            crate::parser::expr_parser()
                .then_ignore(end())
                .parse_recovery(s)
        },
        vec![
            // Literals
            "1",
            // Neg
            "-1",
            // Simple math
            "1 + 1",
            "-1-1",
            "1-1",
            "1 * 1",
            "1 / 1",
            "1 / 1 - 1",
            "1 - 1 / 1 - 1",
            "- 1 - 1 / 1 - 1",
            "1 > -1",
            // TODO +1
            // "+1", // Ser??
            // Comparation
            // "-1 < +1",
            "-1 < 1",
            "-1 < 1",
            "1 == 1",
            "1 < 1",
            "1 - 1 == 1 - 1",
            "1 * 4 == 4 / 1",
            "- 1 *  -4 == 4 / 1",
            "- - 1",
            "--1",
            // Bool algebra
            "true",
            "false",
            "true && true",
            "false && true",
            "1 && 0",
            "5 || -3",
            // Str
            r#""str s??o literals e portanto ainda est??o dentro de expr.""#,
            // Array
            r#"array[2]"#,
            r#"array[2 + 3]"#,
            r#"array[array_index]"#,
            r#"array[array_index + array_indexes[2]] + array[array_index + array_indexes[3]]"#,
        ],
        vec![
            "1+",
            "+1+",
            "1-",
            "/1",
            "*1",
            "1*",
            "1/",
            "1/",
            "-1-",
            "+1+",
            "1 + 1 - 1 / 1 * 1 +",
            "1 + 1 - 1 / 1 *",
            "1 + 1 - 1 /",
            "1 + 1 -",
            "-",
            "+",
            "/",
            "*",
            "1 === 1",
        ],
    )
}

#[test]
pub fn item() {
    test_util::tests(
        |s| {
            crate::parser::statement_block_item_loop_parser()
                .2
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "fn identifier() {}",
            "fn identifier() {body;}",
            "fn identifier(args) {}",
            "fn identifier(args) {body;}",
            "fn identifier(arg) {
                arg;
            }",
            "fn identifier(arg1, arg2) {
                arg1 + arg2;
            }",
        ],
        vec![
            "fn {}",
            "fn (x, y)",
            "fn (x, y) {}",
            "fn identifier",
            "fn identifier()",
            "fn identifier {}",
            "fn identifier {}",
            "fn identifier { body }",
            "fn (args) { body }",
            "fn identifier(arg1, arg2)",
            "identifier(arg1, arg2) { body }",
            "(arg1, arg2)",
            "{ body }",
            "{}",
        ],
    );
}

#[test]
pub fn statement() {
    test_util::tests(
        |s| {
            crate::parser::statement_block_item_loop_parser()
                .0
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            ";",
            "12 + 3;",
            "- 12 +3 - 4 / 5;",
            "add(x, y);",
            "let z = add(x, y);",
            "let x = 1;",
            "let x = 1 + 2;",
            "let x = 1 - 1;",
            "let x = - 1 + 1;",
            "if true {true;}",
            "if true {}",
            "if true {true;} else {false;}",
            "if true {true;} else {}",
            "if add(x, y) {}",
            "if add(x, y) - 5 / 3 {}",
            r#"if "string" {}"#, // This is sintatically valid but will occur in a runtime error
            r#"if "string" {} else {false;}"#, // This is sintatically valid but will occur in a runtime error
        ],
        vec!["12", r#""Termina sem ;""#, "if 0121 {}"],
    );
}

// TODO make some more tests
#[test]
pub fn block() {
    test_util::tests(
        |s| {
            crate::parser::statement_block_item_loop_parser()
                .1
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "{}",
            "{{}}",
            "{;}",
            "{;;;;;}",
            "{{;}}",
            "{{;};}",
            "{{};}",
            "{{;;;;};;;;;}",
            "{add(x);}",
            "{
                add(x);
            }",
            "{
                // coment??o
                add(x);

                // Coment??o denovo
            }",
            "{
                add(x);
                sub(y);
            }",
            "{
            }",
        ],
        vec!["}", "{", "[", "]", "{{ }"],
    );
}

#[test]
pub fn r#loop() {
    test_util::tests(
        |s| {
            crate::parser::statement_block_item_loop_parser()
                .3
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "loop {}",
            "loop {statement();}",
            "loop {
                body;
                // break;
            }",
            "loop {
                body;
                break;
            }",
        ],
        vec!["loop (cond) {{}}", "loop", "{}"],
    );
}

#[test]
#[ignore]
pub fn r#struct() {
    /*     test_util::tests(
        |s| {
            crate::parser::struct_parser()
                .then_ignore(end())
                .parse_recovery_verbose(s)
        },
        vec![
            "struct identifier {{}}",
            "struct identifier {{
                identifier2: type;
            }}",
        ],
        vec![
            "struct {{}}",
            // "struct identifier",
            "identifier {{}}",
        ],
    ); */
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
