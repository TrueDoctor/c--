//! The parser.

use std::iter::Peekable;

use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::util::{CompilerError, CompilerResult, Position};

/// The parser state.
pub struct Parser<I: Iterator<Item = Token>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    /// Creates a new `Parser` from a `Token` iterator.
    pub fn new(tokens: I) -> Self {
        Self {
            iter: tokens.peekable(),
        }
    }

    /// Consumes and returns the next `Token`.
    fn next(&mut self) -> Token {
        self.iter.next().expect("called `Parser::next` after EOF")
    }

    /// Returns the next `Token` without consuming it.
    fn peek(&mut self) -> &Token {
        self.iter.peek().expect("called `Parser::peek` after EOF")
    }

    fn expect(&mut self, tk: &TokenKind) -> CompilerResult<Token> {
        let token = self.next();
        if &token.kind == tk {
            Ok(token)
        } else {
            Err(CompilerError::new(
                format!("expected {}, got {}", tk, token.kind),
                token.pos,
            ))
        }
    }

    fn expect_identifier(&mut self) -> CompilerResult<Ident> {
        let token = self.next();
        let pos = token.pos;
        let name = match &token.kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => {
                return Err(CompilerError::new(
                    format!("expected identifier, got {}", token.kind),
                    token.pos,
                ))
            }
        };
        Ok(Ident { pos, name })
    }

    fn expect_type(&mut self) -> CompilerResult<Type> {
        let token = self.next();
        let pos = token.pos;
        let name = match &token.kind {
            TokenKind::Type(name) => name.clone(),
            _ => {
                return Err(CompilerError::new(
                    format!("expected type, got {}", token.kind),
                    token.pos,
                ))
            }
        };
        Ok(Type { pos, name })
    }

    fn optional(&mut self, tk: &TokenKind) -> bool {
        if &self.peek().kind == tk {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn parse_program(&mut self, name: &str) -> CompilerResult<Program> {
        let mut items = Vec::new();
        loop {
            let item = match self.peek().kind {
                TokenKind::Eof => break,
                TokenKind::Type(_) => {
                    let mut decl = self.parse_declaration()?;
                    let token = self.next();
                    match token.kind {
                        TokenKind::LeftParen => {
                            // function definition
                            let parameters =
                                self.parse_list(Self::parse_declaration, &TokenKind::RightParen)?;
                            let statements = self.parse_block()?;
                            Item {
                                pos: token.pos,
                                kind: ItemKind::Function {
                                    name: decl.name,
                                    return_type: decl.type_,
                                    parameters,
                                    statements,
                                },
                            }
                        }
                        TokenKind::Eq => {
                            // declaration with initialization
                            decl.init = Some(self.parse_expr()?);
                            self.expect(&TokenKind::Semicolon)?;
                            decl.into()
                        }
                        TokenKind::Semicolon => decl.into(),
                        _ => {
                            return Err(CompilerError::new(
                                "expected function definition or declaration",
                                token.pos,
                            ))
                        }
                    }
                }
                _ => self.parse_statement()?.into(),
            };
            items.push(item);
        }
        Ok(Program {
            items,
            name: name.to_string(),
        })
    }

    fn parse_list<T>(
        &mut self,
        p: impl Fn(&mut Self) -> CompilerResult<T>,
        end: &TokenKind,
    ) -> CompilerResult<Vec<T>> {
        let mut elems = Vec::new();
        while &self.peek().kind != end {
            elems.push(p(self)?);
            if !self.optional(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(end)?;
        Ok(elems)
    }

    fn parse_declaration(&mut self) -> CompilerResult<Declaration> {
        let type_ = self.expect_type()?;
        let name = self.expect_identifier()?;
        let init = None;
        Ok(Declaration { type_, name, init })
    }

    fn parse_block(&mut self) -> CompilerResult<Vec<Statement>> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::RightBrace {
            statements.push(self.parse_statement()?);
        }
        self.next();
        Ok(statements)
    }

    fn parse_statement(&mut self) -> CompilerResult<Statement> {
        let token = self.peek();
        let pos = token.pos;
        let kind = match token.kind {
            TokenKind::LeftBrace => StatementKind::Block {
                statements: self.parse_block()?,
            },
            TokenKind::If => {
                self.next();
                self.expect(&TokenKind::LeftParen)?;
                let condition = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                let if_statement = Box::new(self.parse_statement()?);
                let else_statement = if self.optional(&TokenKind::Else) {
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                StatementKind::If {
                    condition,
                    if_statement,
                    else_statement,
                }
            }
            TokenKind::While => {
                self.next();
                self.expect(&TokenKind::LeftParen)?;
                let condition = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                let statement = Box::new(self.parse_statement()?);
                StatementKind::While {
                    condition,
                    statement,
                }
            }
            TokenKind::Repeat => {
                self.next();
                self.expect(&TokenKind::LeftParen)?;
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                let statement = Box::new(self.parse_statement()?);
                StatementKind::Repeat { expr, statement }
            }
            TokenKind::Return => {
                self.next();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::Semicolon)?;
                StatementKind::Return { expr }
            }
            TokenKind::Inline => {
                self.next();
                let token = self.next();
                let code = match token.kind {
                    TokenKind::StringLiteral(value) => value,
                    _ => {
                        return Err(CompilerError::new(
                            format!("expected string literal, got {}", token.kind),
                            token.pos,
                        ));
                    }
                };
                self.expect(&TokenKind::Semicolon)?;
                StatementKind::Inline { code }
            }
            TokenKind::Type(_) => {
                let decl = self.parse_declaration()?;
                self.expect(&TokenKind::Semicolon)?;
                StatementKind::Declaration(decl)
            }
            TokenKind::Identifier(_) => {
                let name = self.expect_identifier()?;
                if self.optional(&TokenKind::LeftParen) {
                    // function call
                    let args = self.parse_list(Self::parse_expr, &TokenKind::RightParen)?;
                    self.expect(&TokenKind::Semicolon)?;
                    StatementKind::Call { name, args }
                } else {
                    let token = self.next();
                    let pos = token.pos;
                    let kind = match token.kind {
                        TokenKind::Eq => AssignOpKind::Eq,
                        TokenKind::PlusEq => AssignOpKind::PlusEq,
                        TokenKind::MinusEq => AssignOpKind::MinusEq,
                        TokenKind::StarEq => AssignOpKind::StarEq,
                        TokenKind::SlashEq => AssignOpKind::SlashEq,
                        TokenKind::PercentEq => AssignOpKind::PercentEq,
                        _ => {
                            return Err(CompilerError::new(
                                "expected function call or assignment",
                                pos,
                            ));
                        }
                    };
                    let op = AssignOp { pos, kind };
                    let expr = self.parse_expr()?;
                    self.expect(&TokenKind::Semicolon)?;
                    StatementKind::Assign { name, op, expr }
                }
            }
            _ => return Err(CompilerError::new("expected statement", pos)),
        };
        Ok(Statement { pos, kind })
    }

    fn parse_expr(&mut self) -> CompilerResult<Expr> {
        // TODO: use precedence climbing / pratt parsing
        Err(CompilerError::new(
            "expressions are not yet supported",
            Position::default(),
        ))
    }
}

pub fn parse_program(tokens: Vec<Token>, program_name: &str) -> CompilerResult<Program> {
    Parser::new(tokens.into_iter()).parse_program(program_name)
}
