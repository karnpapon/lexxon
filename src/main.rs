use std::collections::VecDeque;

const OPCODES: &'static str = ".abcdefghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ";
const HEXDIGITS: &'static str = "0123456789ABCDEF";
const MAXINT: u32 = 0xFFFFFFFF;

struct Lexxon<'a> {
  lines: Vec<&'a str>,
  title: &'a str,
  tokens: Vec<&'a str>,
  stack: VecDeque<u8>
}

impl Lexxon<'_> {
  /// A Melody consists of lines signifying opcodes and hexadecimal numbers.
  ///[a-z] and [G-Z] denote opcodes, while [1-9] and [A-F] denote numbers.
  fn new(&self, melody: &str, mutedlines: &str) -> Lexxon{
    Lexxon {
      lines: melody.split("!").collect(),
      title,
      tokens: self.tokenize(self.lines[1..], mutedlines),
      stack: VecDeque::with_capacity(256),
    }
  }

  fn repr(){}

  fn reset(&self){
    self.stack
  }

  fn tokenize<'a>(&self, lines: [&str], mutedlines: &str) -> Vec<&'a str> {
    const tokens: Vec<&'a str> = Vec::new();
    const STATE_NUMBER: bool = false;
    tokens
  }

  fn expand(){}

  fn compute(){}

}

fn main() {
    println!("Hello, world!");
}
