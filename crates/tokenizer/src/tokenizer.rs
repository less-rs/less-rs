use std::cell::RefCell;
use crate::utils;
use crate::constants;
use crate::constants::TokenType;
// use crate::constants;
// #[warn(dead_code)]
#[derive(Debug)]
pub struct Token(pub TokenType, pub usize, pub usize);

impl Token {
  pub fn new(kind: TokenType, pos: usize, next: usize) -> Token {
    Token(kind, pos, next)
  }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub less: &'a str,
    ignore: bool,
    length: usize,
    pos: RefCell<usize>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source_code: &'a str, ignore_errors: bool) -> Tokenizer<'a> {
        let length = source_code.len();
        Tokenizer {
            less: source_code,
            ignore: ignore_errors,
            length,
            pos: RefCell::new(0),
        }
    }

    #[inline]
    pub fn position(&self) -> usize {
        *self.pos.borrow()
    }

    pub fn end_of_file(&self) -> bool{
        println!("constants{}", constants::SINGLE_QUOTE);
        return true;
    }

    #[inline]
    fn pos_plus_one(&self) {
        self.pos.replace_with(|it| *it + 1);
    }

    pub fn next_token(&self, _ignore_unclosed: bool) -> Token {
        let mut code = utils::char_code_at(self.less, self.position());
        let current_token: Token;
        // let { singlequote } = WORD_MAP;
        match code {
            constants::NEWLINE | constants::SPACE | constants::TAB | constants::CR | constants::FEED => {
                let mut next = self.position();
                loop {
                  next += 1;
                  code = utils::char_code_at(self.less, next);
                  if !(code == constants::SPACE || code == constants::NEWLINE || code == constants::TAB || code == constants::FEED) {
                    break;
                  }
                }
                current_token = Token(TokenType::Space, self.position(), next);
                self.pos.replace(next);
            }
            constants::OPEN_SQUARE | constants::CLOSE_SQUARE | constants::OPEN_CURLY | constants::CLOSE_CURLY | constants::COLON | constants::SEMICOLON | constants::CLOSE_PARENTHESES => {
                let start = self.position();
                current_token = Token(utils::get_token_type(code), start, start + 1);
                self.pos_plus_one();
            }
            _ => {
                current_token = Token(utils::get_token_type(code), 1, 2);
            }
        }
        current_token
    }
}