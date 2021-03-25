//! An S-expression based representation for expressions.

use std::fmt;

use super::Expr;

#[derive(Debug)]
pub enum Sexp {
    Atom(String),
    List(String, Vec<Sexp>),
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(s) => write!(f, "{}", s),
            Self::List(s, v) => {
                write!(f, "({}", s)?;
                for sexp in v {
                    write!(f, " {}", sexp)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl Sexp {
    pub fn from_expr(expr: Expr) -> Self {
        match expr {
            Expr::Binary { left, op, right } => Self::List(
                format!("{}", op.kind),
                vec![Self::from_expr(*left), Self::from_expr(*right)],
            ),
            Expr::Unary { op, right } => {
                Self::List(format!("{}", op.kind), vec![Self::from_expr(*right)])
            }
            Expr::Call { name, args } => {
                Self::List(name.value, args.into_iter().map(Self::from_expr).collect())
            }
            Expr::Move { name } => Self::List("move".to_string(), vec![Self::Atom(name.value)]),
            Expr::Var { name } => Self::Atom(name.value),
            Expr::Int { value, .. } => Self::Atom(format!("{}", value)),
        }
    }
}
