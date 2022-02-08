
#[test]
fn test() {
    let ε = 1e-10;
    let src = std::fs::read_to_string("tests/sample/simple.zeca").unwrap();
    assert!(f64::abs(zeca::eval_value(src).unwrap() - 13.06) < ε)
}
