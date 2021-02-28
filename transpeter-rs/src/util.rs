//! Utilities for the compiler.

use std::fmt;

/// A specialized [`Result`] type for compiler operations.
pub type CompilerResult<T> = Result<T, CompilerError>;

/// The error type for compiler operations.
#[derive(Debug)]
pub struct CompilerError(String);

impl CompilerError {
    /// Creates a new `CompilerError` from the given message.
    pub fn new<S: ToString>(message: S) -> Self {
        Self(message.to_string())
    }

    /// Creates a new `CompilerError` from the given message and a [`Position`].
    pub fn with_pos<S: fmt::Display>(message: S, pos: Position) -> Self {
        Self(format!("{}: {}", pos, message))
    }
}

/// A type to track the position of a token in a program.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
}

impl Position {
    /// Creates a new default `Position`.
    pub fn new() -> Self {
        Self { line: 1 }
    }

    /// Increments the line counter.
    pub fn inc_line(&mut self) {
        self.line += 1;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}", self.line)
    }
}
