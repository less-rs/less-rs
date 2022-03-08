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

pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}