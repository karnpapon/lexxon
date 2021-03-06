#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#[macro_use] mod macros;

use std::env;
use std::iter;
use std::collections::VecDeque;
use std::io::{self, Write};

const OPCODES: &'static str = ".abcdefghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ";
const HEXDIGITS: &'static str = "0123456789ABCDEF";
const MAXINT: i32 = i32::MAX;

#[derive(Debug)]
struct Lexxon {
  lines: Vec<String>,
  title: Option<String>,
  tokens: Vec<String>,
  stack: VecDeque<i32>
}

// example: w3_forever!a13880fa400he!a3kma2kn30g!aCk28!a12k1ld!2fladm!43n
// lines: ["w3_forever", "a13880fa400he", "a3kma2kn30g", "aCk28", "a12k1ld", "2fladm", "43n"]
// title: "w3_forever"
// tokens: ["a", "13880", "f","a","400","h","e", ...]
// stack: [0; 256]

impl Lexxon {
  fn new(melody: &str) -> Lexxon {
    Lexxon {
      lines: melody.split("!").map(|s| s.to_string()).collect(),
      title: None,
      tokens: Vec::new(),
      stack: VecDeque::with_capacity(256),
    }
  }

  fn get_title(&mut self){
    self.title = match self.lines[0].is_empty() {
      true => { None },
      false => { Some(self.lines[0].clone())}
    }
  }

  fn get_tokens(&mut self, mutedlines: Option<String>){
    self.tokens = self.tokenize(&self.lines[1..], mutedlines);
  }

  fn repr(&self) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut leadchar = String::new();

    for i in 0..(self.lines.len()){
      // strip NOPs from end of lines for readability.
      lines.push(self.lines[i].trim_end_matches(".").to_string());
    }

    if let None = self.title {
      leadchar.push_str("!");
    }

    leadchar.push_str("!");
    let l = lines.join("!");
    let ll = l.trim_end_matches("!");
    leadchar.push_str(&ll);

    return leadchar;
  }

  fn reset_stack(&mut self){
    self.stack.clear();
    self.stack = vecdeque![0; 256];
  }

  fn tokenize(&self, lines: &[String], mutedlines: Option<String>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut state_number: bool = false;
    for (_,line) in lines.iter().enumerate(){
      assert!(line.len() <= 16, "only 16 characters per line allowed");

      if mutedlines.is_some(){ continue }

      for c in line.chars() {
        if HEXDIGITS.find(c).is_some() && state_number {
          let len = tokens.len() - 1;
          tokens.get_mut(len).unwrap().push_str(&c.to_string());
        } else if c != '.' {
          tokens.push(c.to_string())
        }
        state_number = HEXDIGITS.find(c).is_some();
      }
      state_number = false;
    }
    tokens
  }

  /// Appends NOPs to all lines for easy editing.
  /// since we need to trim/pad (with dots) within 16 chars range.
  fn expand(&mut self){
    // limited to 16 lines.
    let dots: String = iter::repeat(".").take(16).collect();
    for i in 0..17 {
      match self.lines.get(i) {
        Some(line_value) => {
          let mut temp = line_value.clone();
          temp.push_str(&dots);
          self.lines[i] = temp[..16].to_string(); 
        },
        None => {
          self.lines.push(dots.clone());
        }
      }
      
    }
  }

  fn compute(&mut self, t: i32) -> i32 {
    let stack = &mut self.stack;
    for (_,token) in self.tokens.iter().enumerate() {

      // if not an opcode, must be a number
      if OPCODES.find(token).is_none() {
        let int_base_64 = i32::from_str_radix(token, 16).unwrap();
        stack.push_back(int_base_64);
        stack.pop_front();
      } 

      match token.chars().as_str() {
        "a" => { // OP_T
          stack.push_back(t & MAXINT);
          stack.pop_front().unwrap();
        },
        "b" => { // OP_PUT
          let a = stack.back().unwrap() % 256;
          let pre_last_item = stack.len() - 2;
          stack[(-a-1) as usize] = *stack.get(pre_last_item).unwrap();
          stack.rotate_right(1); 
        },
        "c" => { stack.rotate_right(1); }, // OP_DROP
        "d" => { // OP_MUL
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          stack.push_back((b * a) & MAXINT);
        },
        "e" => { // OP_DIV
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          match b.checked_div(a){
            Some(_) => { stack.push_back((b / a) & MAXINT)},
            None => { stack.push_back(0)}
          } 
        },
        "f" => { // OP_ADD
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1); 
          let idx = b.overflowing_add(a).0;
          stack.push_back(idx & MAXINT);
        },
        "g" => { // OP_SUB
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          let idx = b.overflowing_sub(a).0;
          stack.rotate_right(1); 
          stack.push_back(idx & MAXINT);
        },
        "h" => { // OP_MOD
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          match b.checked_div(a){
            Some(_) => { stack.push_back((b % a) & MAXINT)},
            None => { stack.push_back(0)}
          } 
        },
        "j" => { // OP_LSHIFT
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          if a < 32 {
            stack.push_back((b << a) & MAXINT);
          } else {
            stack.push_back(0);
          }
        },
        "k" => { // OP_RSHIFT
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          if a < 32 {
            stack.push_back((b >> a) & MAXINT);
          } else {
            stack.push_back(0);
          }
        },
        "l" => { // OP_AND
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          stack.push_back((b & a ) & MAXINT);
        },
        "m" => { // OP_OR
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          stack.push_back((b | a ) & MAXINT);
        },
        "n" => { // OP_XOR
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          stack.push_back((b ^ a ) & MAXINT);
        },
        "o" => { // OP_NOT
          let last_item = stack.len() - 1;
          let mut s = stack.get_mut(last_item).unwrap();
          let neg = s.to_owned();
          s = &mut (!neg & MAXINT);
        },
        "p" => { // OP_DUP
          let last_item = stack.len() - 1;
          stack.push_back(stack.get(last_item).unwrap().to_owned());
          stack.pop_front();
        },
        "q" => { // OP_PICK
          // 0 OP_PICK is equivalent to OP_DUP
          // 0xFF OP_PICK is equivalent to 0xFF
          let last_item = stack.len() - 1;
          let mut a = stack.get_mut(last_item).unwrap();
          let aa = a.to_owned();
          let idx = (aa.wrapping_sub(254)) % 256;
          let neg_idx = wrapped_neg_idx(-idx, 256) as usize;
          a = &mut stack[neg_idx];
        },
        "r" => { // OP_SWAP
          let last = stack.len() - 1;
          let before_last = stack.len() - 2;
          stack.swap(last, before_last); 
        },
        "s" => { // OP_LT
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          if b < a {
            stack.push_back(MAXINT);
          } else {
            stack.push_back(0);
          }
        },
        "t" => { // OP_GT
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned(); 
          stack.rotate_right(1);
          if b > a {
            stack.push_back(MAXINT);
          } else {
            stack.push_back(0);
          }
        },
        "u" => {  // OP_EQ
          let a = stack.pop_back().unwrap();
          let b = stack.back().unwrap().to_owned();
          stack.rotate_right(1);
          if b == a {
            stack.push_back(MAXINT);
          } else {
            stack.push_back(0);
          }
        },
        _ => {}
      }
    }
    let result = stack.back().unwrap();
    return *result;
  }
}

// let keymap_conversion = {
//   'a': 't',
//   'b': 'put',
//   'c': 'drop',

//   'd': '*',
//   'e': '/',
//   'f': '+',
//   'g': '-',
//   'h': '%',

//   'j': '<<',
//   'k': '>>',
//   'l': '&',
//   'm': '|',
//   'n': '^',
//   'o': '~',

//   'p': 'dup',
//   'q': 'pick',
//   'r': 'swap',

//   's': '<',
//   't': '>',
//   'u': '=',
//   '/': '//',

//   '!': '\n',
//   '.': ' ',
// };

fn main() {
  // let test_codes = "42_forever t13880+t400%/ t5>>|t6>>^40- tC>>28 t12>>1&* 2+&t*| 43^";
  //                42_forever a13880fa400he a5kma6kn40g aCk28 a12k1ld 2fladm 43n
  // let c: Vec<char> = test_codes.chars().collect();
  // println!("test codes parser: {:?}", c);
  let args: Vec<String> = env::args().collect();
  let codes = &args[1];
  let mut lex = Lexxon::new(codes);
  lex.reset_stack();
  lex.get_title();
  lex.get_tokens(None);
  let mut i = 0;
  loop {
    io::stdout().write(&[(lex.compute(i) as u8)]).unwrap();
    i += 1;
  }
}

fn wrapped_neg_idx(a: i32, n: i32) -> i32 {
  return ((a % n) + n) % n;
}