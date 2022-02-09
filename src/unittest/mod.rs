#[cfg(test)]
use chumsky::{Parser, prelude::end};

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
    ];

    test_util::ok(|s| comment_parser.then_ignore(end()).parse_recovery_verbose(*s), ok_comments.iter().flatten());

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

    test_util::err(|s| comment_parser.then_ignore(end()).parse_recovery_verbose(*s), bad_comments.iter().flatten());
}
