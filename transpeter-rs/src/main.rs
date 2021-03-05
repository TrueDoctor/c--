use std::io::{BufRead, Write};
use std::{fs, io};

use transpeter::ast::pretty_print::pretty_print_ast;
use transpeter::lexer::tokenize;
use transpeter::parser::parse_program;

use clap::{App, Arg};

/// Compiles `input` and prints debug compilation info.
fn debug_compile(input: &str) {
    tokenize(input)
        .and_then(|tokens| {
            println!("[Tokens]");
            for token in &tokens[..(tokens.len() - 1)] {
                println!("{:?}", token.kind);
            }

            parse_program(tokens, "<repl>")
        })
        .map(|ast| {
            println!("[AST]");
            pretty_print_ast(ast);
        })
        .unwrap_or_else(|err| {
            eprintln!("[Error]");
            eprintln!("{}", err);
        })
}

/// Compiles the program in `path` and prints the output to stdout.
fn compile(path: &str) -> io::Result<()> {
    let program = fs::read_to_string(path)?;
    debug_compile(&program);
    Ok(())
}

/// A small REPL to explore the compiler output.
fn repl() -> io::Result<()> {
    // currently prints the tokens
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = stdin.lock().lines();
    loop {
        print!("> ");
        stdout.flush()?;
        match input.next() {
            Some(line) => debug_compile(&line?),
            None => {
                println!();
                break;
            }
        }
    }
    Ok(())
}

/// The `main` function. Implements the CLI.
fn main() -> io::Result<()> {
    // CLI
    let matches = App::new("transpeter")
        .version("0.1")
        .arg(Arg::with_name("input").help("The input file"))
        .get_matches();

    // testing the lexer
    if let Some(path) = matches.value_of("input") {
        compile(&path)?;
    } else {
        repl()?;
    }
    Ok(())
}
