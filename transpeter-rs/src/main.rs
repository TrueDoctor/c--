mod lexer;
mod token;
mod util;

use std::{fs, io};

use lexer::Lexer;

use clap::{App, Arg};

fn main() -> io::Result<()> {
    // CLI
    let matches = App::new("transpeter")
        .version("0.1")
        .arg(
            Arg::with_name("input")
                .help("The input file")
                .required(true),
        )
        .get_matches();

    // testing the lexer
    let path = matches.value_of("input").unwrap();
    let program = fs::read_to_string(path)?;
    println!(
        "{:#?}",
        Lexer::new(&program)
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    );
    Ok(())
}
