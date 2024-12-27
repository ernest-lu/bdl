pub mod ast;
pub mod parser;
#[cfg(test)]
mod parser_test;

use std::env;
use std::fs;

fn main() {
    let file = env::args().nth(1).expect("No file provided");
    let src = fs::read_to_string(file).expect("Failed to read file");
    let prog = parser::parse_program(&src).expect("Failed to parse program");

    println!("{:?}", prog);
}
