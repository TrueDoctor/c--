//! Types for the tokens.

use std::fmt;

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
    /// `true`
    True,
    /// `false`
    False,
    /// `move`
    Move,

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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier(name) => write!(f, "identifier `{}`", name),
            TokenKind::Type(name) => write!(f, "type `{}`", name),
            TokenKind::IntLiteral(value) => write!(f, "integer literal `{}`", value),
            TokenKind::CharLiteral(value) => f
                .write_str("char literal `'")
                .and_then(|_| fmt_char(f, *value))
                .and_then(|_| f.write_str("'`")),
            TokenKind::StringLiteral(value) => f
                .write_str("string literal `\"")
                .and_then(|_| value.iter().try_for_each(|&c| fmt_char(f, c)))
                .and_then(|_| f.write_str("\"`")),
            TokenKind::If => write!(f, "keyword `if`"),
            TokenKind::Else => write!(f, "keyword `else`"),
            TokenKind::While => write!(f, "keyword `while`"),
            TokenKind::Repeat => write!(f, "keyword `repeat`"),
            TokenKind::Return => write!(f, "keyword `return`"),
            TokenKind::Inline => write!(f, "keyword `inline`"),
            TokenKind::And => write!(f, "keyword `and`"),
            TokenKind::Or => write!(f, "keyword `or`"),
            TokenKind::Not => write!(f, "keyword `not`"),
            TokenKind::True => write!(f, "keyword `true`"),
            TokenKind::False => write!(f, "keyword `false`"),
            TokenKind::Move => write!(f, "keyword `move`"),
            TokenKind::LeftParen => write!(f, "`(`"),
            TokenKind::RightParen => write!(f, "`)`"),
            TokenKind::LeftBrace => write!(f, "`{{`"),
            TokenKind::RightBrace => write!(f, "`}}`"),
            TokenKind::Comma => write!(f, "`,`"),
            TokenKind::Semicolon => write!(f, "`;`"),
            TokenKind::Plus => write!(f, "`+`"),
            TokenKind::Minus => write!(f, "`-`"),
            TokenKind::Star => write!(f, "`*`"),
            TokenKind::Slash => write!(f, "`/`"),
            TokenKind::Percent => write!(f, "`%`"),
            TokenKind::Eq => write!(f, "`=`"),
            TokenKind::PlusEq => write!(f, "`+=`"),
            TokenKind::MinusEq => write!(f, "`-=`"),
            TokenKind::StarEq => write!(f, "`*=`"),
            TokenKind::SlashEq => write!(f, "`/=`"),
            TokenKind::PercentEq => write!(f, "`%=`"),
            TokenKind::EqEq => write!(f, "`==`"),
            TokenKind::NotEq => write!(f, "`!=`"),
            TokenKind::Greater => write!(f, "`>`"),
            TokenKind::GreaterEq => write!(f, "`>=`"),
            TokenKind::Less => write!(f, "`<`"),
            TokenKind::LessEq => write!(f, "`<=`"),
            TokenKind::Eof => write!(f, "end of file"),
        }
    }
}

fn fmt_char(f: &mut fmt::Formatter<'_>, c: u8) -> fmt::Result {
    match c {
        0x07 => f.write_str("\\a"),
        0x08 => f.write_str("\\b"),
        0x0C => f.write_str("\\f"),
        0x0A => f.write_str("\\n"),
        0x0D => f.write_str("\\r"),
        0x09 => f.write_str("\\t"),
        0x0B => f.write_str("\\v"),
        0x27 => f.write_str("\\'"),
        0x22 => f.write_str("\\\""),
        0x5C => f.write_str("\\\\"),
        b' ' => f.write_str(" "),
        x if x.is_ascii_graphic() => write!(f, "{}", c as char),
        _ => write!(f, "\\x{:02X}", c),
    }
}
