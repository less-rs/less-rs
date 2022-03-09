use std::hint::unreachable_unchecked;
use crate::constants;
use crate::constants::TokenType;

#[inline]
pub fn char_code_at(s: &str, n: usize) -> char {
    if n >= s.len() {
        '\0'
    } else {
        s.as_bytes()[n] as char
    }
}

pub fn get_token_type(ch: char) -> TokenType {
    match ch {
        constants::OPEN_SQUARE => TokenType::OpenSquare,
        constants::CLOSE_SQUARE => TokenType::CloseSquare,
        constants::OPEN_CURLY => TokenType::OpenCurly,
        constants::CLOSE_CURLY => TokenType::CloseCurly,
        constants::COLON => TokenType::Colon,
        constants::SEMICOLON => TokenType::Semicolon,
        constants::CLOSE_PARENTHESES => TokenType::CloseParentheses,
        _ => unsafe { unreachable_unchecked() },
    }
}

pub fn index_of_word_end(s: &str, start: usize) -> usize {
    let bytes = s.as_bytes();
    let mut i = start;
    let len = bytes.len();

    while i < len {
        match bytes[i] as char {
        '\t' | '\n' | '\u{c}' | '\r' | ' ' | '!' | '"' | '#' | '\'' | '(' | ')' | ':' | ';' | '@'
        | '[' | '\\' | ']' | '{' | '}' => {
            return i;
        }
        '/' => {
            if bytes[i + 1] as char == '*' {
            return i;
            } else {
            i += 1;
            }
        }
        _ => i += 1,
        };
    }
    i
}

pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}