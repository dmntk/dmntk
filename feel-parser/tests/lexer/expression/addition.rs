use dmntk_feel_parser::{Lexer, Token};

fn numeric(n: usize) -> Token {
    Token::Numeric(n.to_string(), String::new(), '+', String::new())
}

#[test]
fn _0001() {
    let mut lexer = Lexer::new("1+2");
    assert_eq!(numeric(1), lexer.next_token().unwrap());
    assert_eq!(Token::Plus, lexer.next_token().unwrap());
    assert_eq!(numeric(2), lexer.next_token().unwrap());
    assert_eq!(Token::Eof, lexer.next_token().unwrap());
}

#[test]
fn _0002() {
    let mut lexer = Lexer::new("  1 +  2");
    assert_eq!(numeric(1), lexer.next_token().unwrap());
    assert_eq!(Token::Plus, lexer.next_token().unwrap());
    assert_eq!(numeric(2), lexer.next_token().unwrap());
    assert_eq!(Token::Eof, lexer.next_token().unwrap());
}

#[test]
fn _0003() {
    let mut lexer = Lexer::new("1 \u{000B} + \n 2");
    assert_eq!(numeric(1), lexer.next_token().unwrap());
    assert_eq!(Token::Plus, lexer.next_token().unwrap());
    assert_eq!(numeric(2), lexer.next_token().unwrap());
    assert_eq!(Token::Eof, lexer.next_token().unwrap());
}
