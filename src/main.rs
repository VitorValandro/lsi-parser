use std::collections::HashSet;
use std::env;
use std::fs;

mod lexer; // Import the lexer module
mod ll1_table;
mod parser; // Import the LL(1) table // Import the parser module
mod token; // Import the token module

use lexer::tokenize;
use parser::parse;
use token::{Terminal, Token, TokenValue};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let keywords: HashSet<String> = [
        "if".to_string(),
        "else".to_string(),
        "return".to_string(),
        "int".to_string(),
        "def".to_string(),
        "num".to_string(),
    ]
    .iter()
    .cloned()
    .collect();

    let mut symbol_table: HashSet<String> = keywords.clone();

    match tokenize(&contents, &keywords, &mut symbol_table) {
        Ok(tokens) => {
            println!("Token List:");
            for token in &tokens {
                println!("{:?}", token);
            }

            match parse(tokens) {
                Ok(_) => println!("Parsing successful."),
                Err(err) => eprintln!("Parsing failed: {}", err),
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

/// Parses the program arguments to get the file path.
fn parse_args(args: &[String]) -> Result<String, String> {
    if args.len() != 2 {
        Err("Usage: <program> <file_path>".to_string())
    } else {
        Ok(args[1].clone())
    }
}
