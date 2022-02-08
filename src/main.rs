use zeca::eval_value;

pub fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let result = eval_value(src).unwrap();
    println!("{result}");
}
