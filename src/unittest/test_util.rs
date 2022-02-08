use std::fmt::Debug;

use chumsky::prelude::Simple;

pub fn ok<F, T, O>(parser: F, iterable: T)
where
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> Result<O, Vec<Simple<char>>>,
{
    for t in iterable {
        println!(">Testing ok: {:?}", t);
        assert!(parser(t).is_ok());
    }
}

pub fn err<F, T, O>(parser: F, iterable: T)
where
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> Result<O, Vec<Simple<char>>>,
{
    for t in iterable {
        println!(">Testing err: {:?}", t);
        assert!(parser(t).is_err());
    }
}

