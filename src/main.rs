use std::fs;
use std::env;
use neon_lang::lex;
use neon_lang::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1].as_str();
    let option = *(&args[2].as_str());

    let code = fs::read_to_string(file_name);

    match option {
        "~" => {
            let tokens = lex(&code.unwrap());
            interpret(tokens);
        }
        _ => {
            eprintln!("Invalid option");
        }
    }
}
