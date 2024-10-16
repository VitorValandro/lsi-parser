/**
 * Analisador léxico simples para identificadores, números inteiros e operadores relacionais
 * Autores:
 * - Vitor Matheus Valandro da Rosa (22102567)
 * - Pedro Henrique Rocha (22100918)      
 *
 * Expressões regulares:
 * - Identificador: [a-zA-Z][a-zA-Z0-9]*
 * - Número inteiro: [0-9]+
 * - Operadores relacionais: ==, !=, >=, <=, >, <
 */
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum TokenType {
    Id,
    Int,
    Relop,
    Keyword,
}

#[derive(Debug)]
enum TokenValue {
    Lexeme(String),
    Number(i32),
    RelopLabel(String),
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    value: TokenValue,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Erro: {}", err);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(file_path).expect("Erro ao ler o arquivo");

    let (keywords, inputs) = parse_file_contents(&contents);
    let keyword_lines = keywords.len(); // Conta as linhas dos keywords

    match tokenize(&inputs, &keywords, keyword_lines) {
        Ok(tokens) => {
            // Saída do programa
            println!("Lista de Tokens:");
            for token in tokens {
                println!("{:?}", token);
            }

            println!("\nTabela de Símbolos:");
            for keyword in &keywords {
                println!("{}", keyword);
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

fn tokenize(
    input: &str,
    keywords: &HashSet<String>,
    keyword_lines: usize,
) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();  // Um iterador que pode ser espiável ex: aabb
    let mut line = keyword_lines + 1; // Ajusta a contagem de linhas
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        // Skip whitespace and newline characters
        if ch.is_whitespace() { // Verifica se há um espaço em branco
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            chars.next();
            continue;
        }

        let mut max_token = None;
        let mut max_length = 0;

        // Tenta identificar um identificador
        let mut temp_chars = chars.clone();
        if let Some(token) = parse_identifier(&mut temp_chars, keywords) {
            if token.lexeme.len() > max_length {
                max_length = token.lexeme.len();
                max_token = Some(token);
            }
        }

        // Tenta identificar um número
        let mut temp_chars = chars.clone();
        if let Some(token) = parse_number(&mut temp_chars) {
            if token.lexeme.len() > max_length {
                max_length = token.lexeme.len();
                max_token = Some(token);
            }
        }

        // Tenta identificar um operador relacional
        let mut temp_chars = chars.clone();
        if let Some(token) = parse_relop(&mut temp_chars) {
            if token.lexeme.len() > max_length {
                max_length = token.lexeme.len();
                max_token = Some(token);
            }
        }

        // Adiciona o token mais longo (maximal_munch) à lista de tokens
        if let Some(token) = max_token {
            for _ in 0..token.lexeme.len() {
                if let Some(ch) = chars.next() {
                    if ch == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
            }
            tokens.push(token);
        } else {
            // Se nenhum token foi reconhecido, usar o wildcard parser
            if let Some(error) = parse_wildcard(&mut chars, line, column) {
                return Err(error);
            }
        }
    }

    Ok(tokens)
}

fn parse_identifier(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    keywords: &HashSet<String>,
) -> Option<Token> {
    let mut lexeme = String::new();
    let mut state = 0;

    while let Some(&ch) = chars.peek() {
        match state {
            0 => {
                if ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) {
                    lexeme.push(ch);
                    chars.next();
                    state = 1;
                } else {
                    break;
                }
            }
            1 => {
                if ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch.is_digit(10) {
                    lexeme.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if lexeme.is_empty() {
        return None; // Rejeita se nenhum identificador foi reconhecido
    }

    // Verifica se o próximo caractere é um delimitador válido
    if let Some(&next_ch) = chars.peek() {
        if next_ch.is_alphanumeric() {
            // Rejeita se o próximo caractere é alfanumérico
            // Porque significa que o token não foi reconhecido completamente e o próximo caractere é parte dele
            return None;
        }
    }

    if keywords.contains(&lexeme) {
        Some(Token {
            token_type: TokenType::Keyword,
            lexeme: lexeme.clone(),
            value: TokenValue::Lexeme(lexeme),
        })
    } else {
        Some(Token {
            token_type: TokenType::Id,
            lexeme: lexeme.clone(),
            value: TokenValue::Lexeme(lexeme),
        })
    }
}

fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {
    let mut lexeme = String::new();
    let mut state = 0;

    while let Some(&ch) = chars.peek() {
        match state {
            0 => {
                if ch.is_digit(10) {
                    lexeme.push(ch);
                    chars.next();
                    state = 1;
                } else {
                    break;
                }
            }
            1 => {
                if ch.is_digit(10) {
                    lexeme.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    if lexeme.is_empty() {
        return None; // Rejeita se nenhum número foi reconhecido
    }

    // Verifica se o próximo caractere é um delimitador válido
    if let Some(&next_ch) = chars.peek() {
        if next_ch.is_alphanumeric() {
            // Rejeita se o próximo caractere é alfanumérico
            // Porque significa que o token não foi reconhecido completamente e o próximo caractere é parte dele
            return None;
        }
    }

    let value = lexeme.parse::<i32>().unwrap();

    Some(Token {
        token_type: TokenType::Int,
        lexeme: lexeme.clone(),
        value: TokenValue::Number(value),
    })
}

fn parse_relop(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {
    let mut lexeme = String::new();
    let mut state = 0;

    while let Some(&ch) = chars.peek() {
        match state {
            0 => {
                if ch == '=' || ch == '!' {
                    lexeme.push(ch);
                    chars.next();
                    state = 1;
                } else if ch == '<' || ch == '>' {
                    lexeme.push(ch);
                    chars.next();
                    state = 2;
                } else {
                    break;
                }
            }
            1 => {
                if ch == '=' {
                    lexeme.push(ch);
                    chars.next();
                    state = 4;
                } else {
                    state = 3;
                }
            }
            2 => {
                if ch == '=' {
                    lexeme.push(ch);
                    chars.next();
                    state = 4;
                } else {
                    state = 3;
                }
            }
            3 => break,
            4 => break,
            _ => break,
        }
    }

    if lexeme.is_empty() {
        return None; // Rejeita se nenhum operador relacional foi reconhecido
    }

    let relop_label = match lexeme.as_str() {
        "==" => "EQ".to_string(),
        "!=" => "NE".to_string(),
        "<" => "LT".to_string(),
        "<=" => "LE".to_string(),
        ">" => "GT".to_string(),
        ">=" => "GE".to_string(),
        _ => "".to_string(),
    };

    Some(Token {
        token_type: TokenType::Relop,
        lexeme: lexeme.clone(),
        value: TokenValue::RelopLabel(relop_label),
    })
}

fn parse_wildcard(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    line: usize,
    column: usize,
) -> Option<String> {
    if let Some(&ch) = chars.peek() {
        chars.next(); // Consome o caractere
        Some(format!(
            "Erro: Caractere não identificado '{}' na linha {}, coluna {}",
            ch, line, column
        ))
    } else {
        None
    }
}

fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() != 2 {
        return Err("O caminho do arquivo de entrada deve ser fornecido como argumento. Por exemplo: cargo run entrada-A.txt");
    }
    let file_path = &args[1];
    Ok(file_path)
}

fn parse_file_contents(contents: &str) -> (HashSet<String>, String) {
    let mut parts = contents.split("%%");
    let keywords_part = parts.next().expect("Erro ao ler as palavras-chave");
    let inputs_part = parts.next().expect("Erro ao ler os inputs");

    let keywords: HashSet<String> = keywords_part
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    (keywords, inputs_part.to_string())
}
