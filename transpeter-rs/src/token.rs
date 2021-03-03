//! Types for the tokens.

use crate::util::Position;

/// An enum representing the different kinds of tokens.
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// An identifier.
    Identifier(String),
    /// A type name (`void` or `int`).
    Type(String),
    /// An integer literal.
    IntLiteral(u8),
    /// An ASCII character literal.
    CharLiteral(u8),
    /// An ASCII string literal.
    StringLiteral(Vec<u8>),

    // keywords

    /// `if`
    If,
    /// `else`
    Else,
    /// `while`
    While,
    /// `repeat`
    Repeat,
    /// `return`
    Return,
    /// `inline`
    Inline,
    /// `and`
    And,
    /// `or`
    Or,
    /// `not`
    Not,

    // separators

    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `,`
    Comma,
    /// `;`
    Semicolon,

    // operators

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `=`
    Eq,
    /// `+=`
    PlusEq,
    /// `-=`
    MinusEq,
    /// `*=`
    StarEq,
    /// `/=`
    SlashEq,
    /// `%=`
    PercentEq,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
    /// `>`
    Greater,
    /// `>=`
    GreaterEq,
    /// `<`
    Less,
    /// `<=`
    LessEq,

    /// End of file.
    Eof,
}

/// A token.
#[derive(Debug)]
pub struct Token {
    /// The kind of token.
    pub kind: TokenKind,
    /// The position of the token.
    pub pos: Position,
}
