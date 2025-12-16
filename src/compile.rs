use std::io::Write;
use crate::sys::token::*;
use crate::sys::*;

pub fn serialize(code: &str) -> Vec<u8> {
    let mut bin = Vec::new();
    for process in lex(code) {
        match process {
            Token::Set(Position(a), Value(b)) => {
                bin.push(0u8);
                bin.push((a.to_le_bytes())[0]);
                bin.push((b.to_le_bytes())[0]);
            }
            Token::Get(Position(a)) => {
                bin.push(1u8);
                bin.push((a.to_le_bytes())[0]);
                bin.push(0u8);
            }
            Token::EOF => break,
        }
    }
    bin
}

pub fn write_bin(bin: &[u8], file: &mut std::fs::File) -> std::io::Result<()> {
    file.write_all(bin)?;
    Ok(())
}

pub fn deserialize(bin: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chunks = bin.chunks_exact(3);
    let bin_tokens = chunks.map(|chunk| {
        <[u8; 3]>::try_from(chunk).unwrap()
    });

    for token in bin_tokens {
        match token {
            [0, a, b] => tokens.push(Token::Set(Position(u8::from_le_bytes([a])), Value(u8::from_le_bytes([b])))),
            [1, a, 0] => tokens.push(Token::Get(Position(u8::from_le_bytes([a])))),
            _ => panic!("Invalid token"),
        }
    }
    tokens.push(Token::EOF);

    tokens
}
