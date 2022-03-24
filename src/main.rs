use zeca::eval_source;

pub fn main() {
    let src = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .expect("Please provide a file to parse and interpret"),
    )
    .expect("Error converting file contents to string");
    let result = eval_source(src).unwrap();
    println!("{:?}", result);
}
