use super::log;

#[test]
pub fn comments() {
    let comment_parser = crate::grammar::CommentsParser::new();

    let ok_line_comments = vec![
        "// this is a comment",
        "//",
        "// ",
        "///",
        "// this is a comment // this still is",
    ];
    let ok_block_comments = vec![
        "/* a block comment */",
        "   /* anoter block comment */     ",
        "/** a double block comment **/",
        "/* should still be valid **/",
        "/*so should this*/",
        "/* nested block comments 
            /*
                Should be fine
             */
        */",
        "/* this is
        multi line */",
        "/** this is
        multi line double **/",
        "/**/",
        "/* /* */",
    ];

    for suite in [ok_line_comments, ok_block_comments] {
        for comment in suite {
            log::debug!("Testing {}", id);
            assert!(identifier_parser.parse(id).is_ok());
        }
    }

    let bad_line_comments = vec!["not a comment", "something before // comment ", "", " "];

    let bad_block_comments = vec![
        "/*/",
        "/* /",
        "/ */",
        "/*",
        "something before /* comment */"
        "/* comment */ something after"
    ];

    for suite in [bad_line_comments, bad_block_comments] {
        for comment in suite {
            log::debug!("Testing {}", id);
            assert!(identifier_parser.parse(id).is_err());
        }
    }
}
