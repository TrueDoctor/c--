pub mod ast;
pub mod code_gen;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod util;

use ast::pretty_print::pretty_print_ast;
use code_gen::{generate_code, Program};
use lexer::tokenize;
use parser::parse_program;

/// Compiles `input` to a [`Program`].
/// If `debug` is true, prints debug compilation info.
pub fn compile(input: &str, name: &str, debug: bool) -> Option<Program> {
    if debug {
        tokenize(input)
            .and_then(|tokens| {
                println!("[Tokens]");
                // ignore EOF token
                for token in &tokens[..(tokens.len() - 1)] {
                    println!("{:?}", token.kind);
                }

                parse_program(tokens.into_iter(), name)
            })
            .and_then(|ast| {
                println!("\n[AST]");
                pretty_print_ast(&ast);

                generate_code(ast)
            })
            .map(|program| {
                println!("\n[Code]");
                println!("{:#?}", program);

                program
            })
            .map_err(|err| {
                eprintln!("\n[Error]");
                eprintln!("{}", err);
            })
            .ok()
    } else {
        tokenize(input)
            .and_then(|tokens| parse_program(tokens.into_iter(), name))
            .and_then(generate_code)
            .map_err(|err| {
                eprintln!("[Error]");
                eprintln!("{}", err);
            })
            .ok()
    }
}
