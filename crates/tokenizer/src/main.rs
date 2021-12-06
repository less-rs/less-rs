use std::fs::read_to_string;
use std::time::Instant;
use tokenizer::Tokenizer;

fn main() {
    println!("less-rs tokenizer123");
    let file = "base.less";
    let less: String = read_to_string(format!("../../assets/{}", file)).unwrap();
    let mut vec = Vec::default();
    let start = Instant::now(
    let processor = Tokenizer::new(&less, false);
    while !processor.end_of_file() {
      vec.push(processor.next_token(false));
    }
    let end = start.elapsed();
    println!("rust: tokenizer/{}: {:?}", file, end);
}
