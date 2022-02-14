use zeca::parser::ast::{Number, Value};

fn parse_file<P>(path: P) -> Value
where
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
    let val = parse_file("tests/examples/good/simple.zeca");
    let val = match val {
        Value::Num(Number::Float(x)) => x,
        _ => panic!(),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

#[test]
fn negation() {
    let expected_value = -3.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/neg.zeca");
    let val = match val {
        Value::Num(Number::Float(x)) => x,
        _ => panic!(),
    };
    assert!(f64::abs(val - expected_value) < delta);
}
