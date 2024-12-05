use std::collections::HashSet;
use std::env;
use std::fs;
/**
 * T3 de compiladores: Parser preditivo
 * Esse programa é um analisador sintático que implementa um parser preditivo para a linguagem de programação LSI-2024-2.
 * O programa lê um arquivo de entrada contendo um programa escrito na linguagem LSI-2024-2 e verifica se o programa está léxica e sintaticamente correto.
 * Autores:
 * - Vitor Matheus Valandro da Rosa (22102567)
 * - Pedro Henrique Nascimento Rocha (22100918)
 */
mod lexer;
mod ll1_table;
mod parser;
mod token;

use lexer::tokenize;
use parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(file_path).expect("Erro ao ler arquivo");

    let keywords: HashSet<String> = [
        "if".to_string(),
        "else".to_string(),
        "return".to_string(),
        "int".to_string(),
        "def".to_string(),
        "num".to_string(),
        "print".to_string(),
    ]
    .iter()
    .cloned()
    .collect();

    let mut symbol_table: HashSet<String> = keywords.clone();

    match tokenize(&contents, &keywords, &mut symbol_table) {
        Ok(tokens) => {
            println!("Lista de tokens:");
            for token in &tokens {
                println!("{:?}", token);
            }

            match parse(tokens) {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("\nParsing falhou: {}", err),
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() != 2 {
        return Err("O caminho do arquivo de entrada deve ser fornecido como argumento. Por exemplo: cargo run entrada.txt");
    }
    let file_path = &args[1];
    Ok(file_path)
}
