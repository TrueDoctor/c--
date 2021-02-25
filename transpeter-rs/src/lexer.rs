use std::{iter::Peekable, str::CharIndices};

use crate::token::*;
use crate::util::*;

pub struct Lexer<'a> {
    program: &'a str,
    iter: Peekable<CharIndices<'a>>,
    pos: Position,
    done: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            iter: program.char_indices().peekable(),
            pos: Position::new(),
            done: false,
        }
    }

    fn token(&self, tt: TokenType, i: usize, j: usize) -> CompilerResult<Token<'a>> {
        Ok(Token {
            token_type: tt,
            value: &self.program[i..j],
            pos: self.pos,
        })
    }

    fn error(&mut self, msg: &str) -> CompilerResult<Token<'a>> {
        self.done = true;
        Err(CompilerError::with_pos(msg, self.pos))
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

    pub fn next_token(&mut self) -> CompilerResult<Token<'a>> {
        // short-circuit if lexer is already done
        if self.done {
            return self.token(TokenType::Eof, 0, 0);
        }

        // consume characters until a token (or an error) is returned
        while let Some((i, next)) = self.iter.next() {
            match next {
                // newline
                '\n' => self.pos.inc_line(),
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
                            // TODO: read inline brainfuck code
                            let k = self.consume_while(|c| c != ';');
                            let mut code = String::new();
                            for c in self.program[j..k].chars() {
                                match c {
                                    '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => code.push(c),
                                    _ => {}
                                }
                            }
                            TokenType::Inline
                        }
                        "void" | "int" => TokenType::Type,
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "not" => TokenType::Not,
                        _ => TokenType::Identifier,
                    };
                    return self.token(tt, i, j);
                }
                // integer literal
                x if x.is_ascii_digit() => {
                    let j = self.consume_while(|c| c.is_ascii_digit());
                    return self.token(TokenType::IntLiteral, i, j);
                }
                // char literal
                '\'' => {} // TODO: implement
                // separators
                '(' => return self.token(TokenType::LeftParen, i, i + 1),
                ')' => return self.token(TokenType::RightParen, i, i + 1),
                '{' => return self.token(TokenType::LeftBrace, i, i + 1),
                '}' => return self.token(TokenType::RightBrace, i, i + 1),
                ',' => return self.token(TokenType::Comma, i, i + 1),
                ';' => return self.token(TokenType::Semicolon, i, i + 1),
                // operators
                '=' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::EqEq, i, i + 2)
                    } else {
                        self.token(TokenType::Eq, i, i + 1)
                    };
                }
                '+' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::PlusEq, i, i + 2)
                    } else {
                        self.token(TokenType::Plus, i, i + 1)
                    };
                }
                '-' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::MinusEq, i, i + 2)
                    } else {
                        self.token(TokenType::Minus, i, i + 1)
                    };
                }
                '*' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::StarEq, i, i + 2)
                    } else {
                        self.token(TokenType::Star, i, i + 1)
                    };
                }
                '/' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::SlashEq, i, i + 2)
                    } else {
                        self.token(TokenType::Slash, i, i + 1)
                    };
                }
                '%' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::PercentEq, i, i + 2)
                    } else {
                        self.token(TokenType::Percent, i, i + 1)
                    };
                }
                '>' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::GreaterEq, i, i + 2)
                    } else {
                        self.token(TokenType::Greater, i, i + 1)
                    };
                }
                '<' => {
                    return if self.consume_if(|c| c == '=') {
                        self.token(TokenType::LessEq, i, i + 2)
                    } else {
                        self.token(TokenType::Less, i, i + 1)
                    };
                }
                '!' => {
                    return if let Some((_, '=')) = self.iter.next() {
                        self.token(TokenType::NotEq, i, i + 2)
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
        self.token(TokenType::Eof, 0, 0)
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = CompilerResult<Token<'a>>;
    type IntoIter = TokenStream<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenStream(self)
    }
}

pub struct TokenStream<'a>(Lexer<'a>);

impl<'a> Iterator for TokenStream<'a> {
    type Item = CompilerResult<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next_token())
    }
}
