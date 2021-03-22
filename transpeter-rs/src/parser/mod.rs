//! The parser.

use std::fmt;
use std::iter::Peekable;

use crate::ast::*;
use crate::token::{Token, TokenKind};
use crate::util::{compiler_error, CompilerResult};

#[cfg(test)]
mod tests;

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

fn err_expected<T>(msg: impl fmt::Display, token: Token) -> CompilerResult<T> {
    compiler_error(token.pos, format!("expected {}, got {}", msg, token.kind))
}

struct Parser<I: Iterator<Item = Token>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    /// Creates a new `Parser` from a `Token` iterator.
    fn new(tokens: I) -> Self {
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
            err_expected(tk, token)
        }
    }

    fn expect_identifier(&mut self) -> CompilerResult<Ident> {
        let token = self.next();
        let pos = token.pos;
        match token.kind {
            TokenKind::Identifier(value) => Ok(Ident { pos, value }),
            _ => err_expected("identifier", token),
        }
    }

    fn expect_type(&mut self) -> CompilerResult<Type> {
        let token = self.next();
        let pos = token.pos;
        match token.kind {
            TokenKind::Type(value) => Ok(Type { pos, value }),
            _ => err_expected("type", token),
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

    fn parse_program(&mut self, name: &str) -> CompilerResult<Program> {
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
                            Item::Function(ItemFunction {
                                name: decl.name,
                                return_type: decl.type_,
                                parameters,
                                statements,
                            })
                        }
                        TokenKind::Eq => {
                            // declaration with initialization
                            decl.init = Some(self.parse_expr()?);
                            self.expect(&TokenKind::Semicolon)?;
                            Item::Statement(Statement::Declaration(decl))
                        }
                        TokenKind::Semicolon => Item::Statement(Statement::Declaration(decl)),
                        _ => return err_expected("function definition or declaration", token),
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
        loop {
            match self.peek().kind {
                TokenKind::RightBrace | TokenKind::Eof => break,
                TokenKind::Type(_) => {
                    let mut decl = self.parse_declaration()?;
                    if self.optional(&TokenKind::Eq) {
                        decl.init = Some(self.parse_expr()?);
                    }
                    self.expect(&TokenKind::Semicolon)?;
                    statements.push(Statement::Declaration(decl));
                }
                _ => statements.push(self.parse_statement()?),
            }
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> CompilerResult<Statement> {
        let token = self.peek();
        let pos = token.pos;
        Ok(match token.kind {
            TokenKind::LeftBrace => Statement::Block(self.parse_block()?),
            TokenKind::If => {
                self.next();
                self.expect(&TokenKind::LeftParen)?;
                let condition = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                let then_statement = Box::new(self.parse_statement()?);
                let else_statement = if self.optional(&TokenKind::Else) {
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                Statement::If {
                    pos,
                    condition,
                    then_statement,
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

                        // check if `code` is valid brainfuck
                        let mut count = 0usize;
                        for &c in &code {
                            if c == b'[' {
                                count += 1;
                            } else if c == b']' {
                                if count == 0 {
                                    return compiler_error(pos, "unexpected ']' in inline code");
                                } else {
                                    count -= 1;
                                }
                            }
                        }
                        if count > 0 {
                            return compiler_error(pos, "missing ']' in inline code");
                        }

                        Statement::Inline { pos, code }
                    }
                    _ => return err_expected("string literal", token),
                }
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
                        _ => return err_expected("function call or assignment", token),
                    };
                    let op = AssignOp { pos, kind };
                    let expr = self.parse_expr()?;
                    self.expect(&TokenKind::Semicolon)?;
                    Statement::Assign { name, op, expr }
                }
            }
            _ => return err_expected("statement", self.next()),
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
            TokenKind::Identifier(value) => {
                let name = Ident { pos, value };
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
            TokenKind::True => Expr::Int { pos, value: 1 },
            TokenKind::False => Expr::Int { pos, value: 0 },
            _ => return err_expected("expression", token),
        })
    }
}

pub fn parse_program<I: Iterator<Item = Token>>(iter: I, program_name: &str) -> CompilerResult<Program> {
    Parser::new(iter).parse_program(program_name)
}
