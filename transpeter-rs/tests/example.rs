use transpeter::*;

const EXAMPLE: &str = r#"
void println(int x) {
    inline "<.>";
}

void assert(int condition) {
    if (not condition) {
        while (true) {} # error
    }
}

# this is a comment
void main() {
    int x = 42;
    int y = 12;
    if (x == y and y == x) {
        int wtf = x + y;
        println(wtf);
    } else {
        println(x * y);
    }
    inline "blub +-><";
    repeat (x) {
        println(x);
    }
    while (not x <= y) {
        x -= 1 + 3 / 4 * 1;
    }
    assert(x == y);
}

main();
"#;

#[test]
fn example() {
    assert!(
        compile(
            EXAMPLE,
            "example",
            CompilerOptions {
                debug: false,
                run: true,
                no_std: false,
            },
        )
        .is_some(),
    );
}

const STDLIB: &str = r#"
int c = get_char();
put_char(c);
put_int(42);
"#;

#[test]
fn stdlib() {
    assert!(
        compile(
            STDLIB,
            "stdlib_test",
            CompilerOptions {
                debug: false,
                run: false,
                no_std: false,
            },
        )
        .is_some(),
    );
}

#[test]
fn no_stdlib() {
    assert!(
        compile(
            STDLIB,
            "no_stdlib_test",
            CompilerOptions {
                debug: false,
                run: false,
                no_std: true,
            },
        )
        .is_none(),
    );
}
