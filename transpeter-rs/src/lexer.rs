use std::{iter::Peekable, str::CharIndices};

use crate::error::*;

#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    Type(String),
    IntLiteral(u8),
    // keywords
    If,
    Else,
    While,
    Repeat,
    Return,
    Inline(String),
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
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

pub struct Lexer<'a> {
    program: &'a str,
    iter: Peekable<CharIndices<'a>>,
    line_number: usize,
    done: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Lexer {
            program,
            iter: program.char_indices().peekable(),
            line_number: 1,
            done: false,
        }
    }

    fn token(&self, tt: TokenType) -> Option<Result<Token, CompilerError>> {
        Some(Ok(Token {
            token_type: tt,
            line: self.line_number,
        }))
    }

    fn error(&mut self, msg: &str) -> Option<Result<Token, CompilerError>> {
        self.done = true;
        Some(Err(CompilerError::new(msg, self.line_number)))
    }

    fn consume_while(&mut self, pred: impl Fn(char) -> bool) -> usize {
        let mut idx = self.program.len();
        while let Some(&(new_idx, c)) = self.iter.peek() {
            if pred(c) {
                self.iter.next();
            } else {
                idx = new_idx;
                break;
            }
        }
        idx
    }

    fn consume_if(&mut self, pred: impl Fn(char) -> bool) -> bool {
        if let Some(&(_, c)) = self.iter.peek() {
            if pred(c) {
                self.iter.next();
                return true;
            }
        }
        false
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, CompilerError>;

    fn next(&mut self) -> Option<Self::Item> {
        // short-circuit if lexer is already done
        if self.done {
            return None;
        }

        // consume characters until a token (or an error) is returned
        while let Some((i, next)) = self.iter.next() {
            match next {
                // newline
                '\n' => self.line_number += 1,
                // whitespace
                x if x.is_ascii_whitespace() => {}
                // comment
                '#' => {
                    self.consume_while(|c| c != '\n');
                }
                // identifier
                x if x == '_' || x.is_ascii_alphabetic() => {
                    let j = self.consume_while(|c| c.is_ascii_alphanumeric());
                    let ident = &self.program[i..j];
                    let tt = match ident {
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "repeat" => TokenType::Repeat,
                        "return" => TokenType::Return,
                        "inline" => {
                            // read inline brainfuck code
                            let k = self.consume_while(|c| c != ';');
                            let mut code = String::new();
                            for c in self.program[j..k].chars() {
                                match c {
                                    '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => code.push(c),
                                    _ => {}
                                }
                            }
                            TokenType::Inline(code)
                        }
                        "void" | "int" => TokenType::Type(ident.to_string()),
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "not" => TokenType::Not,
                        _ => TokenType::Identifier(ident.to_string()),
                    };
                    return self.token(tt);
                }
                // integer literal
                x if x.is_ascii_digit() => {
                    let j = self.consume_while(|c| c.is_ascii_digit());
                    return match self.program[i..j].parse::<u8>().ok() {
                        Some(value) => self.token(TokenType::IntLiteral(value)),
                        None => self.error("integer literal too big"),
                    };
                }
                // char literal
                '\'' => {} // TODO: implement
                // separators
                '(' => return self.token(TokenType::LeftParen),
                ')' => return self.token(TokenType::RightParen),
                '{' => return self.token(TokenType::LeftBrace),
                '}' => return self.token(TokenType::RightBrace),
                ',' => return self.token(TokenType::Comma),
                ';' => return self.token(TokenType::Semicolon),
                // operators
                '=' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::EqEq)
                    } else {
                        self.token(TokenType::Eq)
                    };
                }
                '+' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::PlusEq)
                    } else {
                        self.token(TokenType::Plus)
                    };
                }
                '-' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::MinusEq)
                    } else {
                        self.token(TokenType::Minus)
                    };
                }
                '*' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::StarEq)
                    } else {
                        self.token(TokenType::Star)
                    };
                }
                '/' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::SlashEq)
                    } else {
                        self.token(TokenType::Slash)
                    };
                }
                '%' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::PercentEq)
                    } else {
                        self.token(TokenType::Percent)
                    };
                }
                '>' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::GreaterEq)
                    } else {
                        self.token(TokenType::Greater)
                    };
                }
                '<' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::LessEq)
                    } else {
                        self.token(TokenType::Less)
                    };
                }
                '!' => {
                    return if let Some((_, '=')) = self.iter.next() {
                        self.token(TokenType::NotEq)
                    } else {
                        self.error("unexpected character, expected `!=`")
                    };
                }
                // invalid token
                _ => return self.error("invalid token"),
            }
        }

        // no more characters left
        self.done = true;
        None
    }
}
