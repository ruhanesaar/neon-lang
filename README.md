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

# Installation
Prerequisites: Rust, Git
Steps:
1. `git clone https://github.com/ruhanesaar/neon-lang`
2. `cd neon-lang`
3. `cargo run --release --bin neon ./main`
