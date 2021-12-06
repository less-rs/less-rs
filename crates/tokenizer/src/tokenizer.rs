
pub struct Tokenizer<'a> {
    pub css: &'a str,
}

impl<'a> Tokenizer<'a'> {
    pub fn new(source_code: &'a str, ignore_errors: bool) -> Tokenizer<'a> {
        let length = source_code.len();
        Tokenizer {
            css: source_code,
        }
    }
}