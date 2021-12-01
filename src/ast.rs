#![allow(non_camel_case_types)]
use std::fmt::{Debug, Error, Formatter};

pub enum KEYWORD {
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
    SELFVALUE,
    SELFTYPE,
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

pub enum WEAK_KEYWORD {
    MACRO_RULES,
    UNION,
    STATICLIFETIME,
}

pub enum RESERVED {
    KEYWORD(KEYWORD),
    WEAK_KEYWORD(WEAK_KEYWORD),
}
