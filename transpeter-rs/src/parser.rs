//! The parser.

use std::fmt;
use std::iter::Peekable;

use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::util::{CompilerError, CompilerResult};

fn binary_bp(op: &TokenKind) -> Option<(BinaryOpKind, (u8, u8))> {
    use BinaryOpKind::*;

    Some(match op {
        TokenKind::Plus => (Plus, (9, 10)),
        TokenKind::Minus => (Minus, (9, 10)),
        TokenKind::Star => (Star, (11, 12)),
        TokenKind::Slash => (Slash, (11, 12)),
        TokenKind::Percent => (Percent, (11, 12)),
        TokenKind::EqEq => (EqEq, (7, 8)),
        TokenKind::NotEq => (NotEq, (7, 8)),
        TokenKind::Greater => (Greater, (7, 8)),
        TokenKind::GreaterEq => (GreaterEq, (7, 8)),
        TokenKind::Less => (Less, (7, 8)),
        TokenKind::LessEq => (LessEq, (7, 8)),
        TokenKind::And => (And, (3, 4)),
        TokenKind::Or => (Or, (1, 2)),
        _ => return None,
    })
}

fn unary_bp(op: &TokenKind) -> Option<(UnaryOpKind, u8)> {
    use UnaryOpKind::*;

    Some(match op {
        TokenKind::Plus => (Plus, 13),
        TokenKind::Minus => (Minus, 13),
        TokenKind::Not => (Not, 5),
        _ => return None,
    })
}

fn expected(msg: impl fmt::Display, token: Token) -> CompilerError {
    CompilerError::new(format!("expected {}, got {}", msg, token.kind), token.pos)
}

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
            Err(expected(tk, token))
        }
    }

    fn expect_identifier(&mut self) -> CompilerResult<Ident> {
        let token = self.next();
        let pos = token.pos;
        match token.kind {
            TokenKind::Identifier(name) => Ok(Ident { pos, name }),
            _ => Err(expected("identifier", token)),
        }
    }

    fn expect_type(&mut self) -> CompilerResult<Type> {
        let token = self.next();
        let pos = token.pos;
        match token.kind {
            TokenKind::Type(name) => Ok(Type { pos, name }),
            _ => Err(expected("type", token)),
        }
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
                            Item::Function {
                                name: decl.name,
                                return_type: decl.type_,
                                parameters,
                                statements,
                            }
                        }
                        TokenKind::Eq => {
                            // declaration with initialization
                            decl.init = Some(self.parse_expr()?);
                            self.expect(&TokenKind::Semicolon)?;
                            Item::Statement(Statement::Declaration(decl))
                        }
                        TokenKind::Semicolon => Item::Statement(Statement::Declaration(decl)),
                        _ => return Err(expected("function definition or declaration", token)),
                    }
                }
                _ => Item::Statement(self.parse_statement()?),
            };
            items.push(item);
        }
        let name = name.to_string();
        Ok(Program { items, name })
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
        self.next(); // has kind TokenKind::RightBrace
        Ok(statements)
    }

    fn parse_statement(&mut self) -> CompilerResult<Statement> {
        let token = self.peek();
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::LeftBrace => Statement::Block {
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
                Statement::If {
                    pos,
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
                Statement::While {
                    pos,
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
                Statement::Repeat {
                    pos,
                    expr,
                    statement,
                }
            }
            TokenKind::Return => {
                self.next();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::Semicolon)?;
                Statement::Return { pos, expr }
            }
            TokenKind::Inline => {
                self.next();
                let token = self.next();
                match token.kind {
                    TokenKind::StringLiteral(code) => {
                        self.expect(&TokenKind::Semicolon)?;
                        Statement::Inline { pos, code }
                    }
                    _ => return Err(expected("string literal", token)),
                }
            }
            TokenKind::Type(_) => {
                let mut decl = self.parse_declaration()?;
                if self.optional(&TokenKind::Eq) {
                    decl.init = Some(self.parse_expr()?);
                }
                self.expect(&TokenKind::Semicolon)?;
                Statement::Declaration(decl)
            }
            TokenKind::Identifier(_) => {
                let name = self.expect_identifier()?;
                if self.optional(&TokenKind::LeftParen) {
                    // function call
                    let args = self.parse_list(Self::parse_expr, &TokenKind::RightParen)?;
                    self.expect(&TokenKind::Semicolon)?;
                    Statement::Call { name, args }
                } else {
                    // assignment
                    let token = self.next();
                    let pos = token.pos;
                    let kind = match token.kind {
                        TokenKind::Eq => AssignOpKind::Eq,
                        TokenKind::PlusEq => AssignOpKind::PlusEq,
                        TokenKind::MinusEq => AssignOpKind::MinusEq,
                        TokenKind::StarEq => AssignOpKind::StarEq,
                        TokenKind::SlashEq => AssignOpKind::SlashEq,
                        TokenKind::PercentEq => AssignOpKind::PercentEq,
                        _ => return Err(expected("function call or assignment", token)),
                    };
                    let op = AssignOp { pos, kind };
                    let expr = self.parse_expr()?;
                    self.expect(&TokenKind::Semicolon)?;
                    Statement::Assign { name, op, expr }
                }
            }
            _ => return Err(expected("statement", self.next())),
        })
    }

    fn parse_expr(&mut self) -> CompilerResult<Expr> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> CompilerResult<Expr> {
        // pratt parsing
        // see https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

        // prefix operators
        let mut lhs = if let Some((op, r_bp)) = unary_bp(&self.peek().kind) {
            let token = self.next();
            let expr = self.parse_expr_bp(r_bp)?;
            Expr::Unary {
                op: UnaryOp {
                    pos: token.pos,
                    kind: op,
                },
                right: Box::new(expr),
            }
        } else {
            self.parse_primary()?
        };

        // infix operators
        while let Some((op, (l_bp, r_bp))) = binary_bp(&self.peek().kind) {
            if l_bp < min_bp {
                break;
            }
            let op_token = self.next();
            let rhs = self.parse_expr_bp(r_bp)?;
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: BinaryOp {
                    pos: op_token.pos,
                    kind: op,
                },
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> CompilerResult<Expr> {
        let token = self.next();
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::Identifier(name) => {
                let name = Ident { pos, name };
                if self.optional(&TokenKind::LeftParen) {
                    // function call
                    let args = self.parse_list(Self::parse_expr, &TokenKind::RightParen)?;
                    Expr::Call { name, args }
                } else {
                    // variable
                    Expr::Var { name }
                }
            }
            TokenKind::LeftParen => {
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                expr
            }
            TokenKind::IntLiteral(value) | TokenKind::CharLiteral(value) => {
                Expr::Int { pos, value }
            }
            _ => return Err(expected("expression", token)),
        })
    }
}

pub fn parse_program(tokens: Vec<Token>, program_name: &str) -> CompilerResult<Program> {
    Parser::new(tokens.into_iter()).parse_program(program_name)
}
