use std::{iter::Peekable, str::CharIndices};

use crate::token::*;
use crate::util::*;

pub struct Lexer<'a> {
    program: &'a str,
    // iterator over `program`
    iter: Peekable<CharIndices<'a>>,
    // start of the current token
    start: usize,
    // current position
    pos: Position,
    done: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            iter: program.char_indices().peekable(),
            start: 0,
            pos: Position::new(),
            done: false,
        }
    }

    fn current(&mut self) -> usize {
        self.iter.peek().map(|&(i, _)| i).unwrap_or_else(|| self.program.len())
    }

    fn advance(&mut self) -> Option<char> {
        self.iter.next().map(|(i, c)| {
            self.start = i;
            c
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|&(_, c)| c)
    }

    fn next(&mut self) -> Option<char> {
        self.iter.next().map(|(_, c)| c)
    }

    fn token(&mut self, tt: TokenType) -> CompilerResult<Token<'a>> {
        Ok(Token {
            token_type: tt,
            value: &self.program[self.start..self.current()],
            pos: self.pos,
        })
    }

    fn error(&mut self, msg: &str) -> CompilerResult<Token<'a>> {
        self.done = true;
        Err(CompilerError::with_pos(msg, self.pos))
    }

    fn consume_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.next();
            } else {
                break;
            }
        }
    }

    fn consume_if(&mut self, pred: impl Fn(char) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if pred(c) {
                self.next();
                return true;
            }
        }
        false
    }

    pub fn next_token(&mut self) -> CompilerResult<Token<'a>> {
        // short-circuit if lexer is already done
        if self.done {
            return self.token(TokenType::Eof);
        }

        // consume characters until a token (or an error) is returned
        while let Some(next) = self.advance() {
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
                    self.consume_while(|c| c.is_ascii_alphanumeric());
                    let ident = &self.program[self.start..self.current()];
                    let tt = match ident {
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "repeat" => TokenType::Repeat,
                        "return" => TokenType::Return,
                        "inline" => {
                            // TODO: read inline brainfuck code
                            let j = self.current();
                            self.consume_while(|c| c != ';');
                            let k = self.current();
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
                    return self.token(tt);
                }
                // integer literal
                x if x.is_ascii_digit() => {
                    self.consume_while(|c| c.is_ascii_digit());
                    return self.token(TokenType::IntLiteral);
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
                    return if let Some('=') = self.next() {
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
        self.token(TokenType::Eof)
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
        if self.0.done {
            None
        } else {
            Some(self.0.next_token())
        }
    }
}
