//! The lexer.

use std::fmt;
use std::{iter::Peekable, str::CharIndices};

use crate::token::*;
use crate::util::*;

/// A type containing the lexer state.
pub struct Lexer<'a> {
    /// The input program.
    program: &'a str,
    /// Iterator over `program`.
    iter: Peekable<CharIndices<'a>>,
    /// The current [`Position`].
    pos: Position,
    /// Wether the lexer is done.
    done: bool,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer.
    pub fn new(program: &'a str) -> Self {
        Self {
            program,
            iter: program.char_indices().peekable(),
            pos: Position::new(),
            done: false,
        }
    }

    /// Returns the next `char` without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().map(|&(_, c)| c)
    }

    /// Consumes and returns the next `char`.
    fn next(&mut self) -> Option<char> {
        self.iter.next().map(|(_, c)| c)
    }

    /// Creates a [`Token`] from the token type `tt`.
    #[allow(clippy::unnecessary_wraps)]
    fn token(&mut self, tt: TokenType) -> CompilerResult<Token> {
        Ok(Token {
            token_type: tt,
            pos: self.pos,
        })
    }

    /// Creates a [`CompilerError`] from the message `msg`.
    fn error<T, S: fmt::Display>(&mut self, msg: S) -> CompilerResult<T> {
        self.done = true;
        Err(CompilerError::with_pos(msg, self.pos))
    }

    /// A combinator that consumes `char`s while they satisfy `pred`.
    /// Returns the index of the next `char` after the last consumed one.
    fn consume_while(&mut self, pred: impl Fn(char) -> bool) -> usize {
        while let Some(&(i, c)) = self.iter.peek() {
            if pred(c) {
                self.iter.next();
            } else {
                return i;
            }
        }
        self.program.len()
    }

    /// A combinator that consumes the next `char` if it satisfies the predicate `pred`.
    /// Returns `true` if the `char` was consumed.
    fn consume_if(&mut self, pred: impl Fn(char) -> bool) -> bool {
        if let Some(c) = self.peek() {
            if pred(c) {
                self.next();
                return true;
            }
        }
        false
    }

    /// A combinator that consumes the next `char` if it is equal to `c`.
    /// Returns `true` if the `char` was consumed.
    fn matches(&mut self, c: char) -> bool {
        self.consume_if(|x| x == c)
    }

    /// Consumes a `char` in a char or string literal.
    fn consume_char(&mut self, c: char) -> Result<u8, CompilerError> {
        if c == '\\' {
            if let Some(escape) = self.next() {
                match escape {
                    'a' => return Ok(0x07),  // audible bell
                    'b' => return Ok(0x08),  // backspace
                    'f' => return Ok(0x0C),  // form feed
                    'n' => return Ok(0x0A),  // line feed
                    'r' => return Ok(0x0D),  // carriage return
                    't' => return Ok(0x09),  // horizontal tab
                    'v' => return Ok(0x0B),  // vertical tab
                    '\'' => return Ok(0x27), // single quote
                    '"' => return Ok(0x22),  // double quote
                    '\\' => return Ok(0x5C), // backslash
                    'x' => {
                        if let (Some(a), Some(b)) = (
                            self.next().and_then(|x| x.to_digit(16)),
                            self.next().and_then(|x| x.to_digit(16)),
                        ) {
                            return Ok((a * 16 + b) as u8);
                        }
                    }
                    _ => {}
                };
                self.error(format!("invalid escape sequence: '\\{}'", escape))
            } else {
                self.error("unterminated escape sequence")
            }
        } else if !c.is_ascii() {
            self.error(format!("expected ASCII character, got '{}'", c))
        } else {
            if c == '\n' {
                self.pos.inc_line();
            }
            Ok(c as u8)
        }
    }

    /// Returns the next [`Token`] or a [`CompilerError`].
    pub fn next_token(&mut self) -> CompilerResult<Token> {
        // short-circuit if the lexer is already done
        if self.done {
            return self.token(TokenType::Eof);
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
                    if self.matches('[') {
                        // block comment
                        let mut depth = 1usize;
                        while let Some(c) = self.next() {
                            if c == '\n' {
                                self.pos.inc_line();
                            } else if c == ']' && self.matches('#') {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            } else if c == '#' && self.matches('[') {
                                depth += 1;
                            }
                        }
                        if depth > 0 {
                            return self.error("unterminated block comment");
                        }
                    } else {
                        self.consume_while(|c| c != '\n');
                    }
                }
                // identifier
                x if x == '_' || x.is_ascii_alphabetic() => {
                    let j = self.consume_while(|c| c == '_' || c.is_ascii_alphanumeric());
                    let ident = &self.program[i..j];
                    let tt = match ident {
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "repeat" => TokenType::Repeat,
                        "return" => TokenType::Return,
                        "inline" => TokenType::Inline,
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
                    let lexeme = &self.program[i..j];
                    return match lexeme.parse::<u8>() {
                        Ok(value) => self.token(TokenType::IntLiteral(value)),
                        Err(_) => self.error(format!("integer literal too big: {}", lexeme)),
                    };
                }
                // char literal
                '\'' => {
                    if let Some(c) = self.next() {
                        if c == '\'' || c == '\n' {
                            return self.error("invalid char literal");
                        }
                        let value = self.consume_char(c)?;
                        if let Some('\'') = self.next() {
                            return self.token(TokenType::CharLiteral(value));
                        }
                    }
                    return self.error("unterminated char literal");
                }
                // string literal
                '"' => {
                    let mut buffer = Vec::new();
                    while let Some(c) = self.next() {
                        if c == '"' {
                            return self.token(TokenType::StringLiteral(buffer));
                        }
                        let value = self.consume_char(c)?;
                        buffer.push(value);
                    }
                    return self.error("unterminated string literal");
                }
                // separators
                '(' => return self.token(TokenType::LeftParen),
                ')' => return self.token(TokenType::RightParen),
                '{' => return self.token(TokenType::LeftBrace),
                '}' => return self.token(TokenType::RightBrace),
                ',' => return self.token(TokenType::Comma),
                ';' => return self.token(TokenType::Semicolon),
                // operators
                '=' => {
                    return if self.matches('=') {
                        self.token(TokenType::EqEq)
                    } else {
                        self.token(TokenType::Eq)
                    };
                }
                '+' => {
                    return if self.matches('=') {
                        self.token(TokenType::PlusEq)
                    } else {
                        self.token(TokenType::Plus)
                    };
                }
                '-' => {
                    return if self.matches('=') {
                        self.token(TokenType::MinusEq)
                    } else {
                        self.token(TokenType::Minus)
                    };
                }
                '*' => {
                    return if self.matches('=') {
                        self.token(TokenType::StarEq)
                    } else {
                        self.token(TokenType::Star)
                    };
                }
                '/' => {
                    return if self.matches('=') {
                        self.token(TokenType::SlashEq)
                    } else {
                        self.token(TokenType::Slash)
                    };
                }
                '%' => {
                    return if self.matches('=') {
                        self.token(TokenType::PercentEq)
                    } else {
                        self.token(TokenType::Percent)
                    };
                }
                '>' => {
                    return if self.matches('=') {
                        self.token(TokenType::GreaterEq)
                    } else {
                        self.token(TokenType::Greater)
                    };
                }
                '<' => {
                    return if self.matches('=') {
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
    type Item = CompilerResult<Token>;
    type IntoIter = TokenStream<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenStream(self)
    }
}

/// An iterator over a [`Lexer`].
pub struct TokenStream<'a>(Lexer<'a>);

impl Iterator for TokenStream<'_> {
    type Item = CompilerResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.done {
            None
        } else {
            Some(self.0.next_token())
        }
    }
}
