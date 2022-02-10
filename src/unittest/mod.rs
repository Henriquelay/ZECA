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

    let bad_bools = ["flase", "treu", " ", "t", "r", "u", "e", "f", "a", "l", "s", "e"];
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
