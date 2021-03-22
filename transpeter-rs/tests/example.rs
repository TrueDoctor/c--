use transpeter::compile;

const PROGRAM: &str = r#"
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
    inline "blub +-><[.,]";
    repeat (x) {
        println(x);
    }
    while (not x <= y) {
        x -= 1 + 3 / 4 * 1;
    }
    assert(x == y);
    return 0;
}

main();
"#;

#[test]
fn example() {
    assert!(compile(PROGRAM, "program", false, true).is_some());
}
