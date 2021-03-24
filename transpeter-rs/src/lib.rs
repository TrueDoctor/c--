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

/// Options passed to [`compile`].
#[derive(Clone, Copy, Default)]
pub struct CompilerOptions {
    /// Print debug output.
    pub debug: bool,
    /// Optimize the generated code.
    pub optimize: bool,
    /// Run the program.
    pub run: bool,
    /// Compile without the standard library.
    pub no_std: bool,
}

/// Compiles `input` to a [`Program`].
pub fn compile(input: &str, name: &str, options: CompilerOptions) -> Option<Program> {
    let program = tokenize(input)
        .and_then(|tokens| parse_program(tokens.into_iter(), name))
        .and_then(|ast| {
            if options.debug {
                println!("[AST]");
                pretty_print_ast(&ast);
                println!();
            }

            let std = if options.no_std {
                None
            } else {
                if options.debug {
                    println!("Compiling std...\n");
                }
                Some(
                    compile(
                        STD,
                        "std",
                        CompilerOptions {
                            no_std: true,
                            ..CompilerOptions::default()
                        },
                    )
                    .unwrap(),
                )
            };
            generate_code(ast, std, options.optimize)
        })
        .map(|program| {
            if options.debug {
                println!("[Program]");
                println!("{:#?}", program);
            }

            program
        })
        .map_err(|err| {
            eprintln!("[Error] {}", err);
        })
        .ok()?;
    if options.run {
        brainfuck::run(&program.code);
    }
    Some(program)
}
