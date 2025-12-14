use std::fs;
use std::env;

pub struct Buffer(pub [u8; 256]);
impl Buffer {
    pub fn new() -> Self {
        Self([0; 256])
    }
    pub fn set(&mut self, index: u8, value: u8) {
        (self.0)[index as usize] = value;
    }
    pub fn get(&self, index: u8) -> u8 {
        (self.0)[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_to_buffer_works() {
        let mut buffer = Buffer::new();
        buffer.set(0, 42);
        assert_eq!((buffer.0)[0], 42);
    }

    #[test]
    fn get_from_buffer_works() {
        let mut buffer = Buffer::new();
        (buffer.0)[0] = 42;
        assert_eq!(buffer.get(0), 42);
    }
}

pub mod token {
    #[derive(PartialEq, Eq, Debug)]
    pub struct Position(pub u8);
    #[derive(PartialEq, Eq, Debug)]
    pub struct Value(pub u8);

    #[derive(PartialEq, Eq, Debug)]
    pub enum Token {
        Set(Position, Value),
        Get(Position),
        EOF,
    }
}

use crate::token::*;
pub fn lex(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for raw_line in code.lines() {
        let line = raw_line.split("//").next().unwrap_or("").trim();
    	if line.is_empty() { continue; }
        let split_str = line.split_whitespace().collect::<Vec<&str>>();
        match split_str[0] {
            "GET" => tokens.push(Token::Get(Position(split_str[1].parse::<i32>().unwrap() as u8))),
            "SET" => tokens.push(Token::Set(Position(split_str[1].parse::<i32>().unwrap() as u8), Value(split_str[2].parse::<i32>().unwrap() as u8))),
            _ => {
                dbg!(&line);
                panic!("Unknown Operator");
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}

pub fn interpret(code: Vec<Token>) {
	let mut buf = Buffer::new();
    for line in code {
        match &line {
            Token::Set(Position(pos), Value(val)) => buf.set(*pos, *val),
            Token::Get(Position(pos)) => println!("{}", buf.get(*pos)),
            Token::EOF => break
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].as_str();

    let code = fs::read_to_string(file_name);

    let tokens = lex(&code.unwrap());
    interpret(tokens);
}
