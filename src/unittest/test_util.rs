use std::fmt::Debug;

use chumsky::prelude::Simple;

pub fn ok<F, T, O>(parser: F, iterable: T)
where
    O: Debug,
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> (Option<O>,Vec<Simple<char>>),
{
    for t in iterable {
        println!(">Testing ok: {:?}", t);
        let parsed = parser(t);
        println!("Parse result: {:?}", parsed);
        assert!(parsed.0.is_some());
    }
}

pub fn err<F, T, O>(parser: F, iterable: T)
where
    O: Debug,
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> (Option<O>,Vec<Simple<char>>),
{
    for t in iterable {
        println!(">Testing err: {:?}", t);
        let parsed = parser(t);
        println!("Parse result: {:?}", parsed);
        assert!(parsed.0.is_none());
    }
}
