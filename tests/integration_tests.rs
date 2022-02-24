use zeca::parser::ast::{Literal, Number};

fn parse_file<P>(path: P) -> Literal
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
        Literal::Num(Number::Float(x)) => x,
        _ => panic!("Returned value is not a Literal::Num(Number::Float())"),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

// FIXME block need to return eval of last element
#[test]
fn negation() {
    let expected_value = -3.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/neg.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!("Returned value is not a Literal::Num(Number::Float())"),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

#[ignore]
#[test]
fn fn_loop() {
    let expected_value = 1.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/fn_loop.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!(),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

#[test]
fn conditional() {
    let expected_value = -1;
    let val = parse_file("tests/examples/good/conditional.zeca");
    let val = match val {
        Literal::Num(Number::Integer(x)) => x,
        _ => panic!("Expected value is not Integer type"),
    };
    assert!(expected_value == val);
}

#[test]
fn all_features() {
    let expected_value = 5;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/full_test_1.zeca");
    let val = match val {
        Literal::Num(Number::Integer(x)) => x,
        _ => panic!(),
    };
    assert!(expected_value == val);
}

#[test]
fn expr_simple() {
    let expected_value = 2.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/expr_simple.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!("Returned value is not a Literal::Num(Number::Float())"),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

#[test]
fn string() {
    let expected_value = "henrique, atílio e luana";
    let val = parse_file("tests/examples/good/string.zeca");
    let val = match val {
        Literal::Str(x) => x,
        _ => panic!("Returned value is not a string"),
    };
    assert!(val == expected_value);
}

#[test]
fn bool() {
    let expected_value = true;
    let val = parse_file("tests/examples/good/bool.zeca");
    let val = match val {
        Literal::Bool(x) => x,
        _ => panic!("Value is not a boolean"),
    };
    assert!(expected_value == val);
}

// #[ignore]
// #[test]
// fn expr() {
//     let expected_value = 6.25;
//     let delta = 1e-10;
//     let val = parse_file("tests/examples/good/expr.zeca");
//     let val = match val {
//         Literal::Num(Number::Float(x)) => x,
//         _ => panic!(),
//     };
//     assert!(f64::abs(val - expected_value) < delta);
// }
