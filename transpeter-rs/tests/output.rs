use std::fs::{read, read_dir};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const CARGO: &str = "cargo";
const CARGO_ARGS: [&str; 3] = ["run", "--quiet", "--"];
const TESTS_DIR: &str = "tests/output";
const CMM_EXT: &str = "cmm";

fn check_out(path: &mut PathBuf, stream: &str, output: &[u8], test_name: &str) {
    path.set_extension(stream);
    if let Ok(expected) = read(&path) {
        if output != expected {
            panic!(
                "{}: {} failed\n expected: {:?}\n      got: {:?}",
                test_name,
                stream,
                String::from_utf8_lossy(&expected),
                String::from_utf8_lossy(output),
            );
        }
    }
}

#[test]
fn output() {
    read_dir(TESTS_DIR)
        .unwrap()
        .map(Result::unwrap)
        .filter_map(|entry| {
            let path = entry.path();
            path.extension().filter(|&ext| ext == CMM_EXT)?;
            Some(path)
        })
        .for_each(|mut path| {
            let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
            eprintln!("  {} ...", file_stem);
            let mut child = Command::new(CARGO)
                .args(&CARGO_ARGS)
                .args(&[path.to_str().unwrap(), "--run"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("failed to spawn command");

            // write to stdin
            path.set_extension("stdin");
            if let Ok(input) = read(&path) {
                child
                    .stdin
                    .as_mut()
                    .and_then(|stdin| stdin.write_all(&input).ok())
                    .expect("failed to write to stdin");
            }

            // check stdout and stderr
            let output = child.wait_with_output().expect("failed to wait on child");
            check_out(&mut path, "stdout", &output.stdout, &file_stem);
            check_out(&mut path, "stderr", &output.stderr, &file_stem);
        });
}
