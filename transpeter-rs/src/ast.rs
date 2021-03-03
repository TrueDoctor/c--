//! The abstract syntax tree.

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
        statements: Vec<Stmt>,
    },
    // Const {},
    // Struct {},
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
pub struct Stmt {
    pos: Position,
    kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        if_statement: Box<Stmt>,
        else_statement: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        statement: Box<Stmt>,
    },
    Repeat {
        expr: Expr,
        statement: Box<Stmt>,
    },
    Return {
        expr: Expr,
    },
    Inline {
        code: String,
    },
    Assign {
        var: Ident,
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

#[derive(Debug)]
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
pub struct Expr {
    pos: Position,
    kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOpKind {
    Plus,
    Minus,
    Not,
}
