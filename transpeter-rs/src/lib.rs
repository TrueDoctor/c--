pub mod ast;
pub mod brainfuck;
pub mod code_gen;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod util;

use ast::pretty_print::pretty_print_ast;
use code_gen::{generate_code, Program};
use lexer::tokenize;
use parser::parse_program;

const STD: &str = include_str!("../lib/std.cmm");

pub struct CompilerOptions {
    pub debug: bool,
    pub run: bool,
    pub no_std: bool,
}

/// Compiles `input` to a [`Program`].
/// If `debug` is true, prints debug compilation info.
/// If `run` is true, runs the resulting program.
pub fn compile(input: &str, name: &str, options: CompilerOptions) -> Option<Program> {
    let program = tokenize(input)
        .and_then(|tokens| parse_program(tokens.into_iter(), name))
        .and_then(|ast| {
            if options.debug {
                println!("\n[AST]");
                pretty_print_ast(&ast);
            }

            let std = if options.no_std {
                None
            } else {
                Some(compile(STD, "std", CompilerOptions { no_std: true, ..options }).unwrap())
            };
            generate_code(ast, std)
        })
        .map(|program| {
            if options.debug {
                println!("\n[Program]");
                println!("{:#?}", program);
            }

            program
        })
        .map_err(|err| {
            eprintln!("\n[Error]");
            eprintln!("{}", err);
        })
        .ok()?;
    if options.run {
        brainfuck::run(&program.code);
    }
    Some(program)
}
