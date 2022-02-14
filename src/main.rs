use zeca::eval_source;

pub fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let result = eval_source(src).unwrap();
    println!("{:?}", result);
}
