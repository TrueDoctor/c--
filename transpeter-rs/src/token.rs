use crate::util::Position;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Type,
    IntLiteral,
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

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a str,
    pub pos: Position,
}
