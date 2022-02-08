use std::fmt::Debug;

use chumsky::prelude::Simple;

pub fn ok<F, T>(parser: F, iterable: T)
where
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> Result<(), Vec<Simple<char>>>,
{
    for t in iterable {
        println!(">Testing: {:?}", t);
        assert!(parser(t).is_ok());
    }
}

pub fn err<F, T>(parser: F, iterable: T)
where
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    F: Fn(T::Item) -> Result<(), Vec<Simple<char>>>,
{
    for t in iterable {
        println!(">Testing: {:?}", t);
        assert!(parser(t).is_ok());
    }
}

