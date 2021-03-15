use super::Parser;
use crate::ast::sexp::Sexp;
use crate::ast::*;
use crate::lexer::tokenize;
use crate::token::Token;

fn parse<T>(input: &str, f: impl Fn(&mut Parser<std::vec::IntoIter<Token>>) -> T) -> T {
    let mut p = Parser::new(tokenize(input).unwrap().into_iter());
    f(&mut p)
}

fn parse_sexp(input: &str) -> String {
    tokenize(input)
        .and_then(|tokens| Parser::new(tokens.into_iter()).parse_expr())
        .map(Sexp::from_expr)
        .unwrap()
        .to_string()
}

macro_rules! assert_matches {
    ($expression:expr, $($pattern:pat)|+ $(if $guard:expr)? $(,)?) => {{
        match $expression {
            $($pattern)|+ $(if $guard)? => {}
            e => std::panic!(
                "assertion failed: `matches!(expr, pattern)`
    expr: `{:?}`,
 pattern: `{}`", e, std::stringify!($($pattern)|+ $(if $guard)?)),
        }
    }};
}

#[test]
fn function() {
    let parse_function = |input| {
        parse(input, |p| {
            p.parse_program("")
                .map(|program| program.items.into_iter().next().unwrap())
        })
    };

    assert_matches!(
        parse_function("void f() {}").unwrap(),
        Item::Function(ItemFunction { name, return_type, parameters, .. }) if name.value == "f"
            && return_type.value == "void"
            && parameters.is_empty(),
    );
    assert_matches!(
        parse_function("void f(int a) {}").unwrap(),
        Item::Function(ItemFunction { name, return_type, parameters, .. }) if name.value == "f"
            && return_type.value == "void"
            && parameters.len() == 1,
    );
    assert_matches!(
        parse_function("void f(int a,) {}").unwrap(),
        Item::Function(ItemFunction { name, return_type, parameters, .. }) if name.value == "f"
            && return_type.value == "void"
            && parameters.len() == 1,
    );
    assert_matches!(
        parse_function("void f(int a, int b) {}").unwrap(),
        Item::Function(ItemFunction { name, return_type, parameters, .. }) if name.value == "f"
            && return_type.value == "void"
            && parameters.len() == 2,
    );
    assert_matches!(
        parse_function("void f(int a, int b,) {}").unwrap(),
        Item::Function(ItemFunction { name, return_type, parameters, .. }) if name.value == "f"
            && return_type.value == "void"
            && parameters.len() == 2,
    );

    assert!(parse_function("void f(,) {}").is_err());
    assert!(parse_function("void f()").is_err());
    assert!(parse_function("void f();").is_err());
    assert!(parse_function("void f() f()").is_err());
    assert!(parse_function("void f(int a;) {}").is_err());
    assert!(parse_function("void f(int a = 42) {}").is_err());
    assert!(parse_function("void f(int a = 42;) {}").is_err());
}

#[test]
fn declaration() {
    let parse_declaration = |input| parse(input, Parser::parse_declaration);

    assert_matches!(
        parse_declaration("int a").unwrap(),
        Declaration { type_, name, init: None } if type_.value == "int" && name.value == "a",
    );
    assert_matches!(
        parse_declaration("void a").unwrap(),
        Declaration { type_, name, init: None } if type_.value == "void" && name.value == "a",
    );

    assert!(parse_declaration("string a").is_err());
    assert!(parse_declaration("string int").is_err());
    assert!(parse_declaration("string void").is_err());
    assert!(parse_declaration("int int").is_err());
    assert!(parse_declaration("int void").is_err());
    assert!(parse_declaration("void int").is_err());
    assert!(parse_declaration("void void").is_err());
}

#[test]
fn statement() {
    let parse_statement = |input| parse(input, Parser::parse_statement);

    // declarations
    assert_matches!(
        parse_statement("int a;").unwrap(),
        Statement::Declaration(Declaration { type_, name, init: None })
            if type_.value == "int" && name.value == "a",
    );
    assert_matches!(
        parse_statement("int a = 42;").unwrap(),
        Statement::Declaration(Declaration { type_, name, init: Some(Expr::Int { value: 42, .. }) })
            if type_.value == "int" && name.value == "a",
    );
    assert!(parse_statement("int a").is_err());
    assert!(parse_statement("int a = 42").is_err());

    // blocks
    assert_matches!(
        parse_statement("{}").unwrap(),
        Statement::Block(statements) if statements.is_empty(),
    );
    assert_matches!(
        parse_statement("{ int a; }").unwrap(),
        Statement::Block(statements) if statements.len() == 1,
    );
    assert_matches!(
        parse_statement("{ int a; int b; }").unwrap(),
        Statement::Block(statements) if statements.len() == 2,
    );

    // if
    assert_matches!(
        parse_statement("if (true) f();").unwrap(),
        Statement::If {
            condition: Expr::Int { value: 1, .. },
            else_statement: None,
            ..
        },
    );
    assert_matches!(
        parse_statement("if (true) {}").unwrap(),
        Statement::If {
            condition: Expr::Int { value: 1, .. },
            else_statement: None,
            ..
        },
    );
    assert_matches!(
        parse_statement("if (true) f(); else f();").unwrap(),
        Statement::If {
            condition: Expr::Int { value: 1, .. },
            else_statement: Some(_),
            ..
        },
    );
    assert_matches!(
        parse_statement("if (true) {} else {}").unwrap(),
        Statement::If {
            condition: Expr::Int { value: 1, .. },
            else_statement: Some(_),
            ..
        },
    );
    assert!(parse_statement("if true {}").is_err());
    assert!(parse_statement("if true {} else {}").is_err());
    assert!(parse_statement("if (true);").is_err());
    assert!(parse_statement("if (true); else;").is_err());
    assert!(parse_statement("if (true); else {}").is_err());
    assert!(parse_statement("if (true) {} else;").is_err());
    assert!(parse_statement("else {}").is_err());
    assert!(parse_statement("else f();").is_err());
    assert!(parse_statement("else;").is_err());

    // while
    assert_matches!(
        parse_statement("while (true) f();").unwrap(),
        Statement::While {
            condition: Expr::Int { value: 1, .. },
            ..
        },
    );
    assert_matches!(
        parse_statement("while (true) {}").unwrap(),
        Statement::While {
            condition: Expr::Int { value: 1, .. },
            ..
        },
    );
    assert!(parse_statement("while true {}").is_err());
    assert!(parse_statement("while (true);").is_err());

    // repeat
    assert_matches!(
        parse_statement("repeat (12) f();").unwrap(),
        Statement::Repeat {
            expr: Expr::Int { value: 12, .. },
            ..
        },
    );
    assert_matches!(
        parse_statement("repeat (12) {};").unwrap(),
        Statement::Repeat {
            expr: Expr::Int { value: 12, .. },
            ..
        },
    );
    assert!(parse_statement("repeat 12 {}").is_err());
    assert!(parse_statement("repeat (12);").is_err());

    // return
    assert_matches!(
        parse_statement("return 12;").unwrap(),
        Statement::Return {
            expr: Expr::Int { value: 12, .. },
            ..
        },
    );
    assert!(parse_statement("return 12").is_err());

    // inline
    assert_matches!(
        parse_statement("inline \"+-><[test].,\";").unwrap(),
        Statement::Inline { code, .. } if code == b"+-><[test].,",
    );
    assert!(parse_statement("inline \"\"").is_err());
    assert!(parse_statement("inline \"[\";").is_err());
    assert!(parse_statement("inline \"]\";").is_err());
    assert!(parse_statement("inline \"[[]\";").is_err());
    assert!(parse_statement("inline \"[]]\";").is_err());

    // assignments
    assert_matches!(
        parse_statement("a = 42;").unwrap(),
        Statement::Assign { name, op, expr: Expr::Int { value: 42, .. }}
            if name.value == "a" && matches!(op.kind, AssignOpKind::Eq),
    );
    assert_matches!(
        parse_statement("a += 42;").unwrap(),
        Statement::Assign { name, op, expr: Expr::Int { value: 42, .. }}
            if name.value == "a" && matches!(op.kind, AssignOpKind::PlusEq),
    );
    assert_matches!(
        parse_statement("a -= 42;").unwrap(),
        Statement::Assign { name, op, expr }
            if name.value == "a"
                && matches!(op.kind, AssignOpKind::MinusEq)
                && matches!(expr, Expr::Int { value: 42, .. }),
    );
    assert_matches!(
        parse_statement("a *= 42;").unwrap(),
        Statement::Assign { name, op, expr }
            if name.value == "a"
                && matches!(op.kind, AssignOpKind::StarEq)
                && matches!(expr, Expr::Int { value: 42, .. }),
    );
    assert_matches!(
        parse_statement("a /= 42;").unwrap(),
        Statement::Assign { name, op, expr }
            if name.value == "a"
                && matches!(op.kind, AssignOpKind::SlashEq)
                && matches!(expr, Expr::Int { value: 42, .. }),
    );
    assert_matches!(
        parse_statement("a %= 42;").unwrap(),
        Statement::Assign { name, op, expr }
            if name.value == "a"
                && matches!(op.kind, AssignOpKind::PercentEq)
                && matches!(expr, Expr::Int { value: 42, .. }),
    );
    assert!(parse_statement("a = 42").is_err());
    assert!(parse_statement("(a) = 42;").is_err());
    assert!(parse_statement("a() = 42;").is_err());
    assert!(parse_statement("12 = 42;").is_err());
    assert!(parse_statement("a = b = 42;").is_err());
    assert!(parse_statement("a = (b = 42);").is_err());

    // function calls
    assert_matches!(
        parse_statement("f();").unwrap(),
        Statement::Call { name, args } if name.value == "f" && matches!(
            *args,
            [],
        ),
    );
    assert_matches!(
        parse_statement("f(1);").unwrap(),
        Statement::Call { name, args } if name.value == "f" && matches!(
            *args,
            [Expr::Int { value: 1, .. }],
        ),
    );
    assert_matches!(
        parse_statement("f(1,);").unwrap(),
        Statement::Call { name, args } if name.value == "f" && matches!(
            *args,
            [Expr::Int { value: 1, .. }],
        ),
    );
    assert_matches!(
        parse_statement("f(1, 2);").unwrap(),
        Statement::Call { name, args } if name.value == "f" && matches!(
            *args,
            [Expr::Int { value: 1, .. }, Expr::Int { value: 2, .. }],
        ),
    );
    assert_matches!(
        parse_statement("f(1, 2,);").unwrap(),
        Statement::Call { name, args } if name.value == "f" && matches!(
            *args,
            [Expr::Int { value: 1, .. }, Expr::Int { value: 2, .. }],
        ),
    );
}

#[test]
fn expression() {
    // function calls
    assert_eq!(parse_sexp("f()"), "(f)");
    assert_eq!(parse_sexp("f(1)"), "(f 1)");
    assert_eq!(parse_sexp("f(1,)"), "(f 1)");
    assert_eq!(parse_sexp("f(1, 2)"), "(f 1 2)");
    assert_eq!(parse_sexp("f(1, 2,)"), "(f 1 2)");

    // variables
    assert_eq!(parse_sexp("a"), "a");

    // integer literals
    assert_eq!(parse_sexp("0"), "0");
    assert_eq!(parse_sexp("255"), "255");
    assert_eq!(parse_sexp("'\\x00'"), "0");
    assert_eq!(parse_sexp("'a'"), "97");
    assert_eq!(parse_sexp("'0'"), "48");
    assert_eq!(parse_sexp("true"), "1");
    assert_eq!(parse_sexp("false"), "0");

    // parenthesized expressions
    assert_eq!(parse_sexp("(a + b)"), "(+ a b)");
}

#[test]
fn binary_expression() {
    assert_eq!(parse_sexp("1 + 2"), "(+ 1 2)");
    assert_eq!(parse_sexp("1 - 2"), "(- 1 2)");
    assert_eq!(parse_sexp("1 * 2"), "(* 1 2)");
    assert_eq!(parse_sexp("1 / 2"), "(/ 1 2)");
    assert_eq!(parse_sexp("1 % 2"), "(% 1 2)");
}

#[test]
fn unary_expression() {
    assert_eq!(parse_sexp("+42"), "(+ 42)");
    assert_eq!(parse_sexp("-42"), "(- 42)");
    assert_eq!(parse_sexp("not 42"), "(not 42)");

    assert_eq!(parse_sexp("++42"), "(+ (+ 42))");
    assert_eq!(parse_sexp("+-42"), "(+ (- 42))");
    assert_eq!(parse_sexp("+ not 42"), "(+ (not 42))");
    assert_eq!(parse_sexp("-+42"), "(- (+ 42))");
    assert_eq!(parse_sexp("--42"), "(- (- 42))");
    assert_eq!(parse_sexp("- not 42"), "(- (not 42))");
    assert_eq!(parse_sexp("not +42"), "(not (+ 42))");
    assert_eq!(parse_sexp("not -42"), "(not (- 42))");
    assert_eq!(parse_sexp("not not 42"), "(not (not 42))");
}

#[test]
fn precedence() {
    assert_eq!(parse_sexp("a + b + c"), "(+ (+ a b) c)");
    assert_eq!(parse_sexp("a + b * c"), "(+ a (* b c))");
    assert_eq!(parse_sexp("a + b == c"), "(== (+ a b) c)");
    assert_eq!(parse_sexp("a + b and c"), "(and (+ a b) c)");
    assert_eq!(parse_sexp("a + b or c"), "(or (+ a b) c)");
    assert_eq!(parse_sexp("a * b + c"), "(+ (* a b) c)");
    assert_eq!(parse_sexp("a * b * c"), "(* (* a b) c)");
    assert_eq!(parse_sexp("a * b == c"), "(== (* a b) c)");
    assert_eq!(parse_sexp("a * b and c"), "(and (* a b) c)");
    assert_eq!(parse_sexp("a * b or c"), "(or (* a b) c)");
    assert_eq!(parse_sexp("a == b + c"), "(== a (+ b c))");
    assert_eq!(parse_sexp("a == b * c"), "(== a (* b c))");
    assert_eq!(parse_sexp("a == b == c"), "(== (== a b) c)");
    assert_eq!(parse_sexp("a == b and c"), "(and (== a b) c)");
    assert_eq!(parse_sexp("a == b or c"), "(or (== a b) c)");
    assert_eq!(parse_sexp("a and b + c"), "(and a (+ b c))");
    assert_eq!(parse_sexp("a and b * c"), "(and a (* b c))");
    assert_eq!(parse_sexp("a and b == c"), "(and a (== b c))");
    assert_eq!(parse_sexp("a and b and c"), "(and (and a b) c)");
    assert_eq!(parse_sexp("a and b or c"), "(or (and a b) c)");
    assert_eq!(parse_sexp("a or b + c"), "(or a (+ b c))");
    assert_eq!(parse_sexp("a or b * c"), "(or a (* b c))");
    assert_eq!(parse_sexp("a or b == c"), "(or a (== b c))");
    assert_eq!(parse_sexp("a or b and c"), "(or a (and b c))");
    assert_eq!(parse_sexp("a or b or c"), "(or (or a b) c)");

    assert_eq!(parse_sexp("not a + b"), "(not (+ a b))");
    assert_eq!(parse_sexp("not a * b"), "(not (* a b))");
    assert_eq!(parse_sexp("not a == b"), "(not (== a b))");
    assert_eq!(parse_sexp("not a and b"), "(and (not a) b)");
    assert_eq!(parse_sexp("not a or b"), "(or (not a) b)");
    assert_eq!(parse_sexp("+ a + b"), "(+ (+ a) b)");
    assert_eq!(parse_sexp("+ a * b"), "(* (+ a) b)");
    assert_eq!(parse_sexp("+ a == b"), "(== (+ a) b)");
    assert_eq!(parse_sexp("+ a and b"), "(and (+ a) b)");
    assert_eq!(parse_sexp("+ a or b"), "(or (+ a) b)");
}
