use std::cell::RefCell;
use crate::utils;
use crate::constants::WORD_MAP;
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
        println!("constants{}", WORD_MAP.singlequote);
        return true;
    }

    pub fn next_token(&self, _ignore_unclosed: bool) -> Token {
        let mut code = utils::char_code_at(self.less, self.position());
        let current_token: Token;
        // let { singlequote } = WORD_MAP;
        const singlequote: char = WORD_MAP.singlequote;
        match code {
            singlequote => {
              let mut next = self.position();
              loop {
                next += 1;
                code = utils::char_code_at(self.less, next);
                if !(code == SPACE || code == NEWLINE || code == TAB || code == FEED) {
                  break;
                }
              }
              current_token = Token(TokenType::Space, self.position(), next);
              self.pos.replace(next);
            }
        }
        current_token
    }
}