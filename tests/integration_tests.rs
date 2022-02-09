fn assert_file<P>(path: P)
-> f64 where
    P: AsRef<std::path::Path>,
{
    let content = std::fs::read_to_string(path).expect("Error opening test file");
    let parsed = zeca::eval_source(content);
    println!("Parse result: {:?}", parsed);
    assert!(parsed.is_ok());
    parsed.unwrap()
}

#[test]
fn simple() {
    let expected_value = 13.06;
    let delta = 1e-10;
    let val = assert_file("tests/example/good/simple.zeca");
    assert!(f64::abs(val - expected_value) < delta);
}
