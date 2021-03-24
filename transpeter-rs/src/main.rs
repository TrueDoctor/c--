use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};

use transpeter::{compile, CompilerOptions};

use clap::{App, Arg};

/// A small REPL to explore the compiler output.
fn repl(optimize: bool) -> io::Result<()> {
    // currently prints the tokens
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = stdin.lock().lines();
    loop {
        print!("> ");
        stdout.flush()?;
        match input.next() {
            Some(line) => compile(
                &line?,
                "<repl>",
                CompilerOptions {
                    debug: true,
                    optimize,
                    run: true,
                    no_std: false,
                },
            ),
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
        .arg(Arg::with_name("optimize")
            .short("o")
            .long("optimize")
            .help("Enable optimizations"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .requires("input")
            .help("Prints debugging output"))
        .arg(Arg::with_name("run")
            .short("r")
            .long("run")
            .requires("input")
            .help("Runs the program"))
        .arg(Arg::with_name("no-std")
            .long("no-std")
            .help("Compiles without the standard library"))
        .get_matches();

    if let Some(path) = matches.value_of("input") {
        let program = fs::read_to_string(&path)?;
        let name = Path::file_stem(path.as_ref()).and_then(OsStr::to_str).unwrap();
        let options = CompilerOptions {
            debug: matches.is_present("debug"),
            optimize: matches.is_present("optimize"),
            run: matches.is_present("run"),
            no_std: matches.is_present("no-std"),
        };
        compile(&program, name, options);
    } else {
        repl(matches.is_present("optimize"))?;
    }
    Ok(())
}
