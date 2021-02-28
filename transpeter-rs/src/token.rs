//! Types for the tokens.

use crate::util::Position;

/// An enum representing the token types.
#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    Type(String),
    IntLiteral(u8),
    CharLiteral(u8),
    StringLiteral(Vec<u8>),
    // keywords
    If,
    Else,
    While,
    Repeat,
    Return,
    Inline,
    And,
    Or,
    Not,
    // separators
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    // operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    EqEq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    // end of file
    Eof,
}

/// A token.
#[derive(Debug)]
pub struct Token {
    /// The type of token.
    pub token_type: TokenType,
    /// The position of the token.
    pub pos: Position,
}
