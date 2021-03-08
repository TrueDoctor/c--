use super::Lexer;
use crate::token::TokenKind;
use crate::token::TokenKind::*;
use crate::util::CompilerResult;

fn tokenize(program: &str) -> CompilerResult<Vec<TokenKind>> {
    let mut res = Lexer::new(program)
        .into_iter()
        .map(|res| res.map(|t| t.kind))
        .collect::<CompilerResult<Vec<_>>>();
    if let Ok(v) = res.as_mut() {
        v.pop();
    };
    res
}

#[test]
fn whitespace() {
    assert_eq!(tokenize("").unwrap(), []);
    assert_eq!(tokenize(" \n\r\t").unwrap(), []);
}

#[test]
fn comment() {
    assert_eq!(tokenize("# test").unwrap(), []);
    assert_eq!(tokenize("# test\n").unwrap(), []);
    assert_eq!(tokenize("## test").unwrap(), []);
    assert_eq!(tokenize("## test\n").unwrap(), []);
    assert_eq!(tokenize("#[ test ]#").unwrap(), []);
    assert_eq!(tokenize("#[ test ]#\n").unwrap(), []);
    assert_eq!(tokenize("#[ #[ test ]# ]#").unwrap(), []);
    assert_eq!(tokenize("#[ #[ test ]# ]#\n").unwrap(), []);

    // invalid block comments
    assert!(tokenize("#[ test").is_err());
    assert!(tokenize("#[ #[ test").is_err());
    assert!(tokenize("#[ #[ test ]#").is_err());
    assert!(tokenize("#[ test ]# ]#").is_err());
    assert!(tokenize("#[ test ] #").is_err());
    assert!(tokenize("#[").is_err());
    assert!(tokenize("]#").is_err());
}

#[test]
fn identifier() {
    fn test_identifier(id: &str) {
        assert_eq!(tokenize(id).unwrap(), [Identifier(id.to_string())]);
    }

    test_identifier("origin");
    test_identifier("note");
    test_identifier("intel");

    test_identifier("a");
    test_identifier("A");
    test_identifier("_");

    test_identifier("ab");
    test_identifier("aB");
    test_identifier("a1");
    test_identifier("a_");
    test_identifier("Ab");
    test_identifier("AB");
    test_identifier("A1");
    test_identifier("A_");
    test_identifier("_b");
    test_identifier("_B");
    test_identifier("_1");
    test_identifier("__");

    test_identifier("abc");
    test_identifier("abC");
    test_identifier("ab2");
    test_identifier("ab_");
    test_identifier("aBc");
    test_identifier("aBC");
    test_identifier("aB2");
    test_identifier("aB_");
    test_identifier("a1c");
    test_identifier("a1C");
    test_identifier("a12");
    test_identifier("a1_");
    test_identifier("a_c");
    test_identifier("a_C");
    test_identifier("a_2");
    test_identifier("a__");
    test_identifier("Abc");
    test_identifier("AbC");
    test_identifier("Ab2");
    test_identifier("Ab_");
    test_identifier("ABc");
    test_identifier("ABC");
    test_identifier("AB2");
    test_identifier("AB_");
    test_identifier("A1c");
    test_identifier("A1C");
    test_identifier("A12");
    test_identifier("A1_");
    test_identifier("A_c");
    test_identifier("A_C");
    test_identifier("A_2");
    test_identifier("A__");
    test_identifier("_bc");
    test_identifier("_bC");
    test_identifier("_b2");
    test_identifier("_b_");
    test_identifier("_Bc");
    test_identifier("_BC");
    test_identifier("_B2");
    test_identifier("_B_");
    test_identifier("_1c");
    test_identifier("_1C");
    test_identifier("_12");
    test_identifier("_1_");
    test_identifier("__c");
    test_identifier("__C");
    test_identifier("__2");
    test_identifier("___");

    assert!(tokenize("ß").is_err());
    assert_ne!(tokenize("0a").unwrap(), [Identifier("0a".to_string())]);
}

#[test]
fn keyword() {
    assert_eq!(tokenize("if").unwrap(), [If]);
    assert_eq!(tokenize("else").unwrap(), [Else]);
    assert_eq!(tokenize("while").unwrap(), [While]);
    assert_eq!(tokenize("repeat").unwrap(), [Repeat]);
    assert_eq!(tokenize("return").unwrap(), [Return]);
    assert_eq!(tokenize("inline").unwrap(), [Inline]);
    assert_eq!(tokenize("void").unwrap(), [Type("void".to_string())]);
    assert_eq!(tokenize("int").unwrap(), [Type("int".to_string())]);
    assert_eq!(tokenize("and").unwrap(), [And]);
    assert_eq!(tokenize("or").unwrap(), [Or]);
    assert_eq!(tokenize("not").unwrap(), [Not]);
    assert_eq!(tokenize("true").unwrap(), [True]);
    assert_eq!(tokenize("false").unwrap(), [False]);
}

#[test]
fn integer_literal() {
    assert_eq!(tokenize("-1").unwrap(), [Minus, IntLiteral(1)]);
    assert_eq!(tokenize("0").unwrap(), [IntLiteral(0)]);
    assert_eq!(tokenize("255").unwrap(), [IntLiteral(255)]);

    assert!(tokenize("256").is_err());
    assert!(tokenize("99999999999999999999").is_err());
}

#[test]
fn char_literal() {
    assert_eq!(tokenize("'a'").unwrap(), [CharLiteral(b'a')]);
    assert_eq!(tokenize("' '").unwrap(), [CharLiteral(b' ')]);
    assert_eq!(tokenize("'\t'").unwrap(), [CharLiteral(b'\t')]);
    assert_eq!(tokenize("','").unwrap(), [CharLiteral(b',')]);

    assert!(tokenize("'").is_err());
    assert!(tokenize("'\n'").is_err());
    assert!(tokenize("''").is_err());
    assert!(tokenize("'''").is_err());

    // escape sequences
    assert_eq!(tokenize("'\\a'").unwrap(), [CharLiteral(7)]);
    assert_eq!(tokenize("'\\b'").unwrap(), [CharLiteral(8)]);
    assert_eq!(tokenize("'\\f'").unwrap(), [CharLiteral(12)]);
    assert_eq!(tokenize("'\\n'").unwrap(), [CharLiteral(10)]);
    assert_eq!(tokenize("'\\r'").unwrap(), [CharLiteral(13)]);
    assert_eq!(tokenize("'\\t'").unwrap(), [CharLiteral(9)]);
    assert_eq!(tokenize("'\\v'").unwrap(), [CharLiteral(11)]);
    assert_eq!(tokenize("'\\''").unwrap(), [CharLiteral(39)]);
    assert_eq!(tokenize("'\\\"'").unwrap(), [CharLiteral(34)]);
    assert_eq!(tokenize("'\\\\'").unwrap(), [CharLiteral(92)]);
    assert_eq!(tokenize("'\\x12'").unwrap(), [CharLiteral(0x12)]);
    assert_eq!(tokenize("'\\xab'").unwrap(), [CharLiteral(0xAB)]);
    assert_eq!(tokenize("'\\xAB'").unwrap(), [CharLiteral(0xAB)]);

    // invalid escape sequences
    assert!(tokenize("'\\c'").is_err());
    assert!(tokenize("'\\x'").is_err());
    assert!(tokenize("'\\x1'").is_err());
    assert!(tokenize("'\\xa'").is_err());
    assert!(tokenize("'\\x123'").is_err());
    assert!(tokenize("'\\xabc'").is_err());
    assert!(tokenize("'\\xgg'").is_err());
    assert!(tokenize("'\\xaz'").is_err());
}

#[test]
fn string_literal() {
    assert_eq!(tokenize("\"\"").unwrap(), [StringLiteral(vec![])]);
    assert_eq!(
        tokenize("\"Hello, world!\"").unwrap(),
        [StringLiteral(Vec::from(*b"Hello, world!"))],
    );
    assert_eq!(tokenize("\"\t\n\"").unwrap(), [StringLiteral(vec![9, 10])]);

    assert!(tokenize("\"").is_err());
    assert!(tokenize("\"\"\"").is_err());

    // escape sequences
    assert_eq!(tokenize("\"\\a\"").unwrap(), [StringLiteral(vec![7])]);
    assert_eq!(tokenize("\"\\b\"").unwrap(), [StringLiteral(vec![8])]);
    assert_eq!(tokenize("\"\\f\"").unwrap(), [StringLiteral(vec![12])]);
    assert_eq!(tokenize("\"\\n\"").unwrap(), [StringLiteral(vec![10])]);
    assert_eq!(tokenize("\"\\r\"").unwrap(), [StringLiteral(vec![13])]);
    assert_eq!(tokenize("\"\\t\"").unwrap(), [StringLiteral(vec![9])]);
    assert_eq!(tokenize("\"\\v\"").unwrap(), [StringLiteral(vec![11])]);
    assert_eq!(tokenize("\"\\'\"").unwrap(), [StringLiteral(vec![39])]);
    assert_eq!(tokenize("\"\\\"\"").unwrap(), [StringLiteral(vec![34])]);
    assert_eq!(tokenize("\"\\\\\"").unwrap(), [StringLiteral(vec![92])]);
    assert_eq!(tokenize("\"\\x42\"").unwrap(), [StringLiteral(vec![0x42])]);

    // invalid escape sequences
    assert!(tokenize("\"\\c\"").is_err());
    assert!(tokenize("\"\\x\"").is_err());
    assert!(tokenize("\"\\x1\"").is_err());
    assert!(tokenize("\"\\xa\"").is_err());
    assert!(tokenize("\"\\xgg\"").is_err());
    assert!(tokenize("\"\\xaz\"").is_err());
}

#[test]
fn separator_operator() {
    assert_eq!(tokenize("(").unwrap(), [LeftParen]);
    assert_eq!(tokenize(")").unwrap(), [RightParen]);
    assert_eq!(tokenize("{").unwrap(), [LeftBrace]);
    assert_eq!(tokenize("}").unwrap(), [RightBrace]);
    assert_eq!(tokenize(",").unwrap(), [Comma]);
    assert_eq!(tokenize(";").unwrap(), [Semicolon]);

    assert_eq!(tokenize("=").unwrap(), [Eq]);
    assert_eq!(tokenize("==").unwrap(), [EqEq]);
    assert_eq!(tokenize("+").unwrap(), [Plus]);
    assert_eq!(tokenize("+=").unwrap(), [PlusEq]);
    assert_eq!(tokenize("-").unwrap(), [Minus]);
    assert_eq!(tokenize("-=").unwrap(), [MinusEq]);
    assert_eq!(tokenize("*").unwrap(), [Star]);
    assert_eq!(tokenize("*=").unwrap(), [StarEq]);
    assert_eq!(tokenize("/").unwrap(), [Slash]);
    assert_eq!(tokenize("/=").unwrap(), [SlashEq]);
    assert_eq!(tokenize("%").unwrap(), [Percent]);
    assert_eq!(tokenize("%=").unwrap(), [PercentEq]);
    assert_eq!(tokenize(">").unwrap(), [Greater]);
    assert_eq!(tokenize(">=").unwrap(), [GreaterEq]);
    assert_eq!(tokenize("<").unwrap(), [Less]);
    assert_eq!(tokenize("<=").unwrap(), [LessEq]);
    assert_eq!(tokenize("!=").unwrap(), [NotEq]);
}
