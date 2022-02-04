#[test]
pub fn arithmetic_or_logical_expression() {
    let expr_parser = crate::grammar::ArithmeticOrLogicalExpressionParser::new();
    let ok_expr = [
        "123 + 123",
        "123 - 123",
        "123 * 123",
        "123 / 123",
        "123 % 123",
        "123 ^ 123",
        "123 & 123",
        "123 | 123",
        "123 << 123",
        "123 >> 123",
    ];
    for id in ok_expr {
        println!("Testing {}", id);
        assert!(expr_parser.parse(id).is_ok());
    }

    let bad_expr = [
        "",
        " ",
        "sla",
        ":)",
        "123 +",
        "123 -",
        "123 *",
        "123 /",
        "123 %",
        "123 ^",
        "123 &",
        "123 |",
        "123 <<",
        "123 >>",
        "123",
        "&",
        "+-*/",
    ];
    for id in bad_expr {
        println!("Testing {}", id);
        assert!(expr_parser.parse(id).is_err());
    }
}
