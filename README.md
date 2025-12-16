# neon-lang
A Programming Language Written in Rust.

Note: This is a work in progress.

# Tutorial
You have 256 changeable 8-bit values, which can be accessed and modified using the programming language.

## Syntax
The syntax of the language is designed to be straightforward and intuitive, with a focus on readability and ease of use.
It goes like this:
```text
<procedure (all CAPS)> <argument(s)>
```

### Get
Retrieve a value from the buffer.
```text
GET <position>
```

### Set
Modify a value from the buffer.
```text
SET <position> <value>
```

### Comments
Comments are denoted by the double-forward-slash (//).
```text
// This is a comment.
```

## Example Program
```
// ./main
// A test Neon file

SET 20 19
GET 20 // 19
SET 64 21
GET 64 // 21
GET 12 // 0
```
There is a ./main file in the repository that you can run to test the language.
There is also a ./main.bin file that is a compiled version of the above.

# Installation
Prerequisites: Rust, Git
Steps:
1. `git clone https://github.com/ruhanesaar/neon-lang`
2. `cd neon-lang`
3. `cargo run --release -- --bin neon ./<neon_file>`

# Compiler
The compiler is also written in Rust (neocomp).
It takes a Neon file as input, and outputs a binary file (bytecode).
There is a one-to-one correspondence between the compiled binary and the code.

## How to run
Steps:
1. `git clone https://github.com/ruhanesaar/neon-lang`
2. `cd neon-lang`
3. `cargo run --release -- neocomp <neon_file> <output_file>`

## How it works
You wont usually need to modify or see the compiled binary (but if you want to, you can use a hex editor like [Hexed.it](https://hexed.it/)).

The compiler reads the Neon file line by line, and converts it into a binary format.

### Procedures
As we saw in the tutorial, the procedures are like the functions in other languages.
The procedure identifier is the first byte of every 3 bytes (3 bytes = 1 line) of a binary file.
The next 2 bytes are the arguments the procedure takes.
If the procedure takes no arguments, the last 2 bytes are 0.
If the procedure takes 1 argument, the last byte are 0.
And so on.

#### Procedure identifiers
| Procedure | Identifier |
| --------- | ---------- |
| GET       | 0x01       |
| SET       | 0x02       |


# Virtual Machine
The virtual machine is written in Rust (neovm).
It takes a binary file (bytecode) as input, and executes it like it is a normal Neon program.

## How to run
Steps:
1. `git clone https://github.com/ruhanesaar/neon-lang`
2. `cd neon-lang`
3. `cargo run --release -- neovm <binary_file>`

You can test this on the ./main.bin file.
