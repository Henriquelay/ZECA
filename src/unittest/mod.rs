#[cfg(test)]
use chumsky::{prelude::end, Parser};

mod test_util;

#[test]
fn comment() {
    use crate::parser::comment as comment_parser;
    let comment_parser = comment_parser();

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
            comment_parser
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
        |s| comment_parser.then_ignore(end()).parse_recovery_verbose(*s),
        bad_comments.iter().flatten(),
    );
}

#[test]
pub fn bool() {
    let bool_parser = crate::parser::boolean;

    let ok_bools = vec!["false", "true"];
    test_util::ok(
        |s| {
            bool_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        ok_bools.iter(),
    );

    let bad_bools = [
        "flase", "treu", " ", "t", "r", "u", "e", "f", "a", "l", "s", "e",
    ];
    test_util::err(
        |s| {
            bool_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        bad_bools.iter(),
    );
}

#[test]
pub fn int() {
    // TODO: Separate parser for floats
    let int_parser = crate::parser::number;

    let ok_ints = vec![
        "0", "123i32", "123usize", "123u32", "0usize", // "-1", // ?
        "1isize", "2usize",
        "123_u32",
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
            int_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        ok_ints.iter(),
    );

    let bad_ints = [
        // "0.1", // Should be a bad integer as soon as the parsers are separated
        "0,1",
        "0invalidSuffix",
        "123AFB43",
        "0b_",
        "0b____",
    ];

    test_util::err(
        |s| {
            int_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        bad_ints.iter(),
    );
}

#[test]
pub fn float() {
    // TODO: Separate parser for floats
    let float_parser = crate::parser::number;

    let ok_floats = vec![
        "2.0", "2.", // "12E+99",
        // "12.01E+99",
        "123.0f64", "0.1f64", "0.1f32", "5f32",
        // "12E+99_f64",
        // "12.01E+99_f64",
    ];

    test_util::ok(
        |s| {
            float_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        ok_floats.iter(),
    );

    let bad_floats = vec![
        "2", "2f16", "2f8", "2.f64", // "2.E+99"
    ];

    test_util::err(
        |s| {
            float_parser()
                .repeated()
                .then_ignore(end())
                .parse_recovery_verbose(*s) // ???
        },
        bad_floats.iter(),
    );
}
