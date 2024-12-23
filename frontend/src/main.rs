pub mod ast;
pub mod parser;
#[cfg(test)]
mod parser_test;

use std::env;
use std::fs;

fn main() {
    parser::BdlParser::parse()
}
