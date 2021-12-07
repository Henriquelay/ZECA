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

impl TryFrom<&str> for Keyword {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "as" => Ok(Keyword::AS),
            "break" => Ok(Keyword::BREAK),
            "const" => Ok(Keyword::CONST),
            "continue" => Ok(Keyword::CONTINUE),
            "crate" => Ok(Keyword::CRATE),
            "else" => Ok(Keyword::ELSE),
            "enum" => Ok(Keyword::ENUM),
            "extern" => Ok(Keyword::EXTERN),
            "false" => Ok(Keyword::FALSE),
            "fn" => Ok(Keyword::FN),
            "for" => Ok(Keyword::FOR),
            "if" => Ok(Keyword::IF),
            "impl" => Ok(Keyword::IMPL),
            "in" => Ok(Keyword::IN),
            "let" => Ok(Keyword::LET),
            "loop" => Ok(Keyword::LOOP),
            "match" => Ok(Keyword::MATCH),
            "mod" => Ok(Keyword::MOD),
            "move" => Ok(Keyword::MOVE),
            "mut" => Ok(Keyword::MUT),
            "pub" => Ok(Keyword::PUB),
            "ref" => Ok(Keyword::REF),
            "return" => Ok(Keyword::RETURN),
            "self" => Ok(Keyword::SELF_VALUE),
            "Self" => Ok(Keyword::SELF_TYPE),
            "static" => Ok(Keyword::STATIC),
            "struct" => Ok(Keyword::STRUCT),
            "super" => Ok(Keyword::SUPER),
            "trait" => Ok(Keyword::TRAIT),
            "true" => Ok(Keyword::TRUE),
            "type" => Ok(Keyword::TYPE),
            "unsafe" => Ok(Keyword::UNSAFE),
            "use" => Ok(Keyword::USE),
            "where" => Ok(Keyword::WHERE),
            "while" => Ok(Keyword::WHILE),
            _ => Err(format!("'{}' is not a valid Keyword.", value)),
        }
    }
}

pub enum WeakKeyword {
    MACRO_RULES,
    UNION,
    STATIC_LIFETIME,
}

impl TryFrom<&str> for WeakKeyword {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "macro_rules" => Ok(WeakKeyword::MACRO_RULES),
            "union" => Ok(WeakKeyword::UNION),
            "'static" => Ok(WeakKeyword::STATIC_LIFETIME),
            _ => Err(format!("'{}' is not a valid WeakKeyword.", value)),
        }
    }
}

pub enum Reserved {
    KEYWORD(Keyword),
    WEAK_KEYWORD(WeakKeyword),
}

pub enum Literal<'a> {
    CHARACTER(char),
    STR(&'a str),
}

pub struct Identifier<'a>(pub &'a str);

pub enum IdentifierOrReserved<'a> {
    Identifier(Identifier<'a>),
    Reserved(Reserved),
}
