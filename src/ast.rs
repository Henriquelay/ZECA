#![allow(non_camel_case_types)]
use std::fmt::{Debug, Error, Formatter};

pub enum Keyword {
    AS,
    BREAK,
    CONST,
    CONTINUE,
    CRATE,
    ELSE,
    ENUM,
    EXTERN,
    FALSE,
    FN,
    FOR,
    IF,
    IMPL,
    IN,
    LET,
    LOOP,
    MATCH,
    MOD,
    MOVE,
    MUT,
    PUB,
    REF,
    RETURN,
    SELF_VALUE,
    SELF_TYPE,
    STATIC,
    STRUCT,
    SUPER,
    TRAIT,
    TRUE,
    TYPE,
    UNSAFE,
    USE,
    WHERE,
    WHILE,
}

pub enum WeakKeyword {
    MACRO_RULES,
    UNION,
    STATIC_LIFETIME,
}

pub enum Reserved {
    KEYWORD(Keyword),
    WEAK_KEYWORD(WeakKeyword),
}

pub struct Identifier<'input>(pub &'input str);

pub enum IdentifierOrReserved<'input> {
    Identifier(Identifier<'input>),
    Reserved(Reserved),
}
