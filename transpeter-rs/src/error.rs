#[derive(Debug)]
pub struct CompilerError(String);

impl CompilerError {
    pub fn new(msg: &str, line: usize) -> Self {
        Self(format!("line {}: {}", line, msg))
    }
}
