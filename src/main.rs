#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn lexer() {
        
    }

    #[test]
    fn parser() {

    }
}
