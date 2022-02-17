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

// FIXME block need to return eval of last element
#[test]
fn simple() {
    let expected_value = 13.06;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/simple.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!(),
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
        _ => panic!(),
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

#[ignore]
#[test]
fn conditional() {
    let expected_value = 1.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/conditional.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!(),
    };
    assert!(f64::abs(val - expected_value) < delta);
}

#[ignore]
#[test]
fn all_features() {
    let expected_value = 6.;
    let delta = 1e-10;
    let val = parse_file("tests/examples/good/all_features.zeca");
    let val = match val {
        Literal::Num(Number::Float(x)) => x,
        _ => panic!(),
    };
    assert!(f64::abs(val - expected_value) < delta);
}


// #[ignore]
// #[test]
// fn bool() {
//     let expected_value = false;
//     let delta = 1e-10;
//     let val = parse_file("tests/examples/good/bool.zeca");
//     let val = match val {
//         Literal::Num(Number::Float(x)) => x,
//         _ => panic!(),
//     };
//     assert!(f64::abs(val - expected_value) < delta);
// }


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
