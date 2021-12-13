
// #[warn(non_snake_case)]
#[warn(dead_code)]
pub struct WordType {
    pub singlequote: char,
    pub doublequote: char,
    pub backslash: char,
    pub slash: char,
    pub newline: char,
    pub space: char,
    pub feed: char,
    pub tab: char,
    pub cr: char,
    pub opensquare: char,
    pub closesquare: char,
    pub openparentheses: char,
    pub closeparentheses: char,
    pub opencurly: char,
    pub closecurly: char,
    pub semicolon: char,
    pub asterisk: char,
    pub colon: char,
    pub at: char,
}

pub const WORD_MAP: WordType = WordType {
    singlequote: '\'',
    doublequote: '"',
    backslash: '\\',
    slash: '/',
    newline: '\n',
    space: ' ',
    feed: '\u{c}',
    tab: '\t',
    cr: '\r',
    opensquare: '[',
    closesquare: ']',
    openparentheses: '(',
    closeparentheses: ')',
    opencurly: '{',
    closecurly: '}',
    semicolon: ',',
    asterisk: '*',
    colon: ':',
    at: '@',
};

#[derive(Debug)]
pub enum TokenType {
    OpenParentheses,
    CloseParentheses,
    Space,
    Word,
    String,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Semicolon,
    Colon,
    Comment,
    AtWord,
    Brackets,
}