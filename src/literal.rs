mod literal {
    use chumsky::{text::{self, TextParser}, Parser, prelude::Simple};
    use crate::Expr;

    pub fn integer(radix: u32) -> impl Parser<char, Expr, Error = Simple<char>> {
        text::int(radix)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .padded()
    }
}
            
