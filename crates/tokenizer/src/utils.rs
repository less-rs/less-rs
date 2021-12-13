#[inline]
pub fn char_code_at(s: &str, n: usize) -> char {
    if n >= s.len() {
        '\0'
    } else {
        s.as_bytes()[n] as char
    }
}

pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}