pub const SINGLE_QUOTE: char = '\'';
pub const DOUBLE_QUOTE: char = '"';
pub const BACKSLASH: char = '\\';
pub const SLASH: char = '/';

pub const NEWLINE: char = '\n';
pub const SPACE: char = ' ';
pub const FEED: char = '\u{c}'; // \f
pub const TAB: char = '\t';
pub const CR: char = '\r';

pub const OPEN_SQUARE: char = '[';
pub const CLOSE_SQUARE: char = ']';
pub const OPEN_PARENTHESES: char = '(';
pub const CLOSE_PARENTHESES: char = ')';
pub const OPEN_CURLY: char = '{';
pub const CLOSE_CURLY: char = '}';

pub const SEMICOLON: char = ';';
pub const ASTERISK: char = '*';
pub const COLON: char = ':';
pub const AT: char = '@';

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