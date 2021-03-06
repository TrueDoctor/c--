//! The abstract syntax tree.

pub mod pretty_print;

use std::fmt;

use crate::util::Position;

#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
    pub pos: Position,
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    Function {
        name: Ident,
        return_type: Type,
        parameters: Vec<Declaration>,
        statements: Vec<Statement>,
    },
    // Const {},
    // Struct {},
    Statement(Statement),
}

#[derive(Debug)]
pub struct Declaration {
    pub type_: Type,
    pub name: Ident,
    pub init: Option<Expr>,
}

#[derive(Debug)]
pub struct Type {
    pub pos: Position,
    pub name: String,
}

#[derive(Debug)]
pub struct Ident {
    pub pos: Position,
    pub name: String,
}

// statements

#[derive(Debug)]
pub struct Statement {
    pub pos: Position,
    pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
    Declaration(Declaration),
    Block {
        statements: Vec<Statement>,
    },
    If {
        condition: Expr,
        if_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    While {
        condition: Expr,
        statement: Box<Statement>,
    },
    Repeat {
        expr: Expr,
        statement: Box<Statement>,
    },
    Return {
        expr: Expr,
    },
    Inline {
        code: Vec<u8>,
    },
    Assign {
        name: Ident,
        op: AssignOp,
        expr: Expr,
    },
    Call {
        name: Ident,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub struct AssignOp {
    pub pos: Position,
    pub kind: AssignOpKind,
}

#[derive(Clone, Copy, Debug)]
pub enum AssignOpKind {
    Eq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
}

// expressions

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Call {
        name: Ident,
        args: Vec<Expr>,
    },
    Var {
        name: Ident,
    },
    Int {
        pos: Position,
        value: u8,
    },
}

#[derive(Debug)]
pub struct BinaryOp {
    pub pos: Position,
    pub kind: BinaryOpKind,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOpKind {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    And,
    Or,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub pos: Position,
    pub kind: UnaryOpKind,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOpKind {
    Plus,
    Minus,
    Not,
}

// trait implementations

impl From<Declaration> for Statement {
    fn from(decl: Declaration) -> Self {
        Self {
            pos: decl.type_.pos,
            kind: StatementKind::Declaration(decl),
        }
    }
}

impl From<Statement> for Item {
    fn from(stmt: Statement) -> Self {
        Self {
            pos: stmt.pos,
            kind: ItemKind::Statement(stmt),
        }
    }
}

impl From<Declaration> for Item {
    fn from(decl: Declaration) -> Self {
        Into::<Statement>::into(decl).into()
    }
}

impl fmt::Display for AssignOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AssignOpKind::*;

        f.write_str(match self {
            Eq => "=",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
        })
    }
}

impl fmt::Display for BinaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOpKind::*;

        f.write_str(match self {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            EqEq => "==",
            NotEq => "!=",
            Greater => ">",
            GreaterEq => ">=",
            Less => "<",
            LessEq => "<=",
            And => "and",
            Or => "or",
        })
    }
}

impl fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UnaryOpKind::*;

        f.write_str(match self {
            Plus => "+",
            Minus => "-",
            Not => "not",
        })
    }
}
