use std::collections::HashSet;
use std::env;
use std::fs;

mod lexer; // Import the lexer module

use lexer::{tokenize, Token};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Erro: {}", err);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(file_path).expect("Erro ao ler o arquivo");

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
            println!("Lista de Tokens:");
            for token in tokens {
                println!("{:?}", token);
            }

            println!("\nTabela de Símbolos:");
            for symbol in &symbol_table {
                println!("{}", symbol);
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
        Err("Uso: <programa> <caminho_para_arquivo>".to_string())
    } else {
        Ok(args[1].clone())
    }
}
