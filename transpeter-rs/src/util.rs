use std::fmt;

#[derive(Debug)]
pub struct CompilerError(String);

impl CompilerError {
    pub fn new(message: &str, pos: Position) -> Self {
        Self(format!("{}: {}", pos, message))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    line: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 1 }
    }

    pub fn inc_line(&mut self) {
        self.line += 1;
    }
}

// TODO:
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}", self.line)
    }
}
