use std::fs::read;
use std::env;
use neon_lang::sys::interpret;
use neon_lang::compile::deserialize;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let bin = read(file_name).expect("Failed to read binary");
    let tokens = deserialize(&bin);
    interpret(tokens);
}