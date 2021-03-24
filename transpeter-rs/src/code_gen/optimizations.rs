use std::num::Wrapping;

use crate::brainfuck;

enum Instruction {
    Move(isize),
    Add(Wrapping<i8>),
    PutChar,
    GetChar,
    Loop(Vec<Instruction>),
}

use Instruction::*;

fn to_instructions(chars: &mut impl Iterator<Item = char>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    macro_rules! adjust {
        ($instr:ident, $expr:expr) => {
            match instructions.last_mut() {
                Some($instr(n)) => {
                    if *n == -($expr) {
                        instructions.pop();
                    } else {
                        *n += $expr;
                    }
                }
                _ => instructions.push($instr($expr)),
            }
        };
    }

    while let Some(c) = chars.next() {
        match c {
            '+' => adjust!(Add, Wrapping(1)),
            '-' => adjust!(Add, Wrapping(-1)),
            '>' => adjust!(Move, 1),
            '<' => adjust!(Move, -1),
            '.' => instructions.push(PutChar),
            ',' => instructions.push(GetChar),
            '[' => {
                let loop_instructions = Loop(to_instructions(chars));
                // a loop after another loop will never be run
                if !matches!(instructions.last(), Some(Loop(_))) {
                    instructions.push(loop_instructions);
                }
            }
            ']' => return instructions,
            _ => {}
        }
    }
    instructions
}

fn from_instructions(buffer: &mut String, instructions: Vec<Instruction>) {
    for instr in instructions {
        match instr {
            Move(n) => {
                if n >= 0 {
                    for _ in 0..n {
                        buffer.push('>');
                    }
                } else {
                    for _ in n..0 {
                        buffer.push('<');
                    }
                }
            }
            Add(n) => {
                let n = n.0;
                if n >= 0 {
                    for _ in 0..n {
                        buffer.push('+');
                    }
                } else {
                    for _ in n..0 {
                        buffer.push('-');
                    }
                }
            }
            PutChar => buffer.push('.'),
            GetChar => buffer.push(','),
            Loop(instructions) => {
                buffer.push('[');
                from_instructions(buffer, instructions);
                buffer.push(']');
            }
        }
    }
}

/// Optimizes the given code. This makes a few assumptions:
/// * 8-bit wrapping cells
/// * cells to the left of the start are not accessed
/// * there are not more than `isize::MAX` consecutive '>' or '<'
pub fn optimize_code(code: &mut String) {
    // "><" => ""
    // "<>" => ""
    // "+-" => ""
    // "-+" => ""
    // "[1][2]" => "[1]"

    debug_assert!(brainfuck::is_valid(code));

    let mut chars = code.chars();
    let instructions = to_instructions(&mut chars);
    *code = String::new();
    from_instructions(code, instructions)
}
