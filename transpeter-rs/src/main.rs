use std::io::{BufRead, Write};
use std::{fs, io};

use transpeter::lexer::tokenize;
use transpeter::token::*;

use clap::{App, Arg};

/// Compiles the program in `path` and prints the output to stdout.
fn compile(path: &str) -> io::Result<()> {
    let program = fs::read_to_string(path)?;
    match tokenize(&program) {
        Ok(tokens) => println!("{:#?}", tokens),
        Err(err) => eprintln!("{:?}", err),
    }
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
            Some(line) => match tokenize(&line?) {
                Ok(mut tokens) => {
                    assert_eq!(tokens.pop().map(|token| token.kind), Some(TokenKind::Eof));
                    for token in tokens {
                        println!("{}", token.kind);
                    }
                }
                Err(err) => eprintln!("{:?}", err),
            },
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
