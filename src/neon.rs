use std::fs;
use std::env;
use neon_lang::sys::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].as_str();

    let code = fs::read_to_string(file_name);

    let tokens = lex(&code.unwrap());
    interpret(tokens);
}
