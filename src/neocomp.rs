use std::env;
use std::io::Write;
use std::fs::{self, OpenOptions};
use neon_lang::compile::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].as_str();
    let bin_name = &args[2].as_str();
    let mut bin = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(bin_name)
        .expect("Failed to open binary file");

    let code = fs::read_to_string(file_name).expect("Failed to read file");

    write_bin(&serialize(&code), &mut bin).unwrap();

    bin.flush().unwrap();
    dbg!(bin);
}
