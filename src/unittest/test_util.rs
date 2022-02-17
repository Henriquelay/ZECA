use std::fmt::Debug;

use chumsky::prelude::Simple;

pub fn ok<P, T, O>(parser: P, iterable: T) -> Vec<O>
where
    O: Debug,
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    P: Fn(T::Item) -> (Option<O>, Vec<Simple<char>>),
{
    iterable
        .into_iter()
        .map(|t| {
            println!(">Testing ok: {:?}", t);
            let parsed = parser(t);
            println!("Parse result: {:?}", parsed);
            assert!(parsed.0.is_some());
            parsed.0.unwrap()
        })
        .collect()
}

pub fn err<P, T, O>(parser: P, iterable: T)
where
    O: Debug,
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    P: Fn(T::Item) -> (Option<O>, Vec<Simple<char>>),
{
    for t in iterable {
        println!(">Testing err: {:?}", t);
        let parsed = parser(t);
        println!("Parse result: {:?}", parsed);
        assert!(parsed.0.is_none());
    }
}

pub fn tests<P, T, O>(parser: P, ok_entry: T, bad_entry: T)
where
    O: Debug,
    T: IntoIterator,
    T::Item: AsRef<str> + Debug,
    P: Fn(T::Item) -> (Option<O>, Vec<Simple<char>>) + Clone,
{
    ok(parser.clone(), ok_entry);

    err(parser.clone(), bad_entry);
}
