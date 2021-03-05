//! Utilities for the compiler.

use std::fmt;

/// A specialized [`Result`] type for compiler operations.
pub type CompilerResult<T> = Result<T, CompilerError>;

/// The error type for compiler operations.
#[derive(Debug)]
pub struct CompilerError {
    pos: Position,
    message: String,
}

impl CompilerError {
    /// Creates a new `CompilerError` from the given message and a [`Position`].
    pub fn new<S: fmt::Display>(message: S, pos: Position) -> Self {
        Self {
            pos,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pos, self.message)
    }
}

/// A type to track the position of a token in a program.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
}

impl Position {
    /// Increments the line counter.
    pub fn inc_line(&mut self) {
        self.line += 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}", self.line)
    }
}
