#[cfg(test)]
use chumsky::Parser;

mod test_util;

#[test]
fn comment() {
    use zeca::parser::comment as comment_parser;
    let comment_parser = comment_parser();

    let ok_comments = [
        vec![
            "// this is a comment",
            "//",
            "// ",
            "///",
            "// this is a comment // this still is",
        ],
        vec![
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
            "/* nested block comments
            /*
                Should be fine
             */
        */",
        ],
    ];

    test_util::ok(|s| comment_parser.parse(*s), ok_comments.iter().flatten());

    let bad_comments = [
        vec![
            "not a comment",
            "something before // comment ",
            "/incomplete",
            "/incomplete/",
            "",
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

    test_util::err(|s| comment_parser.parse(*s), bad_comments.iter().flatten());
}
