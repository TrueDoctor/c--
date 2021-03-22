use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};

use transpeter::compile;

use clap::{App, Arg};

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
            Some(line) => compile(&line?, "<repl>", true, true),
            None => {
                println!();
                break;
            }
        };
    }
    Ok(())
}

/// The `main` function. Implements the CLI.
fn main() -> io::Result<()> {
    use std::ffi::OsStr;

    // CLI
    let matches = App::new("transpeter")
        .version("0.1")
        .arg(Arg::with_name("input").help("The input file"))
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .requires("input")
                .help("Turn on debugging output"),
        )
        .arg(
            Arg::with_name("run")
                .long("run")
                .requires("input")
                .help("Runs the program"),
        )
        .get_matches();

    if let Some(path) = matches.value_of("input") {
        let program = fs::read_to_string(&path)?;
        let name = Path::file_stem(path.as_ref())
            .and_then(OsStr::to_str)
            .unwrap();
        let debug = matches.is_present("debug");
        let run = matches.is_present("run");
        compile(&program, name, debug, run);
    } else {
        repl()?;
    }
    Ok(())
}
