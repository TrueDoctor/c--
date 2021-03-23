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
pub struct CompilerOptions {
    /// Print debug output.
    pub debug: bool,
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
                println!("\n[AST]");
                pretty_print_ast(&ast);
            }

            let std = if options.no_std {
                None
            } else {
                if options.debug {
                    println!("Compiling std...");
                }
                Some(
                    compile(
                        STD,
                        "std",
                        CompilerOptions {
                            debug: false,
                            run: false,
                            no_std: true,
                        },
                    )
                    .unwrap(),
                )
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
