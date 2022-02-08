#[test]
fn simple() {
    let expected_value = 13.06;
    let delta = 1e-10;
    let src = r#"let five = 5.03;
    let eight = 3 + five;
    fn add x y = x + y;
    add(five, eight)
    "#;
    assert!(f64::abs(zeca::eval_source(src.into()).unwrap() - expected_value) < delta)
}

#[test]
fn comments() {
    let expected_value = 13.06;
    let delta = 1e-10;
    let src = r#"let five = 5.03;
    // comentei
    let eight = 3 + five;
    /* comentei
    multi // linha
    * safoda *\  */
    fn add x y = x + y;
    add(five, eight)"#;
    assert!(f64::abs(zeca::eval_source(src.into()).unwrap() - expected_value) < delta)
}
