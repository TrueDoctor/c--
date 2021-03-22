//! A simple brainfuck interpreter.

use std::io;
use std::io::{Read, Write};
use std::num::Wrapping;

/// The maximum number of cells used by [`run`].
pub const N: usize = 30_000;

enum Condition {
    Zero,
    NotZero,
}

impl Condition {
    fn check(&self, value: Wrapping<u8>) -> bool {
        match self {
            Self::Zero => value.0 == 0,
            Self::NotZero => value.0 != 0,
        }
    }
}

enum Instruction {
    Plus,
    Minus,
    Left,
    Right,
    PutChar,
    GetChar,
    JumpIf(Condition, usize),
}

/// Runs a brainfuck program.
///
/// It uses [`N`] 8-bit wrapping cells.
///
/// # Panics
///
/// Panics if the program is invalid or an IO error occurs.
pub fn run(program: &str) {
    use Condition::*;
    use Instruction::*;

    let mut loop_stack = Vec::new();
    let mut code = Vec::new();
    for c in program.chars() {
        match c {
            '+' => code.push(Plus),
            '-' => code.push(Minus),
            '<' => code.push(Left),
            '>' => code.push(Right),
            '.' => code.push(PutChar),
            ',' => code.push(GetChar),
            '[' => {
                loop_stack.push(code.len());
                code.push(Plus); // dummy instruction, will get replaced
            }
            ']' => {
                let start = loop_stack.pop().expect("unmatched ']'");
                let end = code.len();
                code.push(JumpIf(NotZero, start));
                code[start] = JumpIf(Zero, end);
            }
            _ => {} // ignore other characters
        }
    }
    if !loop_stack.is_empty() {
        panic!("unmatched '['");
    }

    let mut cells = [Wrapping(0u8); 30_000];
    let mut ptr = 0;
    let mut i = 0;
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    while i < code.len() {
        match &code[i] {
            Plus => cells[ptr] += Wrapping(1),
            Minus => cells[ptr] -= Wrapping(1),
            Left => ptr -= 1,
            Right => ptr += 1,
            PutChar => {
                print!("{}", cells[ptr].0 as char);
                stdout.flush().unwrap();
            }
            GetChar => {
                let c = stdin.lock().bytes().next().map(|res| res.unwrap()).unwrap_or(255);
                cells[ptr] = Wrapping(c);
            }
            JumpIf(cond, new_i) => {
                if cond.check(cells[ptr]) {
                    i = *new_i;
                }
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    const HELLO: &str = "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.";

    #[test]
    #[should_panic]
    fn invalid_left_bracket() {
        run("[");
    }

    #[test]
    #[should_panic]
    fn invalid_right_bracket() {
        run("]");
    }

    #[test]
    fn hello_world() {
        run(HELLO);
    }
}
