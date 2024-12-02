#![allow(dead_code, unused)]
/**
 * Analisador léxico simples para identificadores, números inteiros, operadores relacionais,
 * operadores aritméticos, parênteses, e strings.
 * Autores:
 * - Vitor Matheus Valandro da Rosa (22102567)
 * - Pedro Henrique Nascimento Rocha (22100918)
 *
 * Expressões regulares:
 * - Identificador: [a-zA-Z][a-zA-Z0-9]*
 * - Número inteiro: [0-9]+
 * - Operadores relacionais: ==, !=, >=, <=, >, <
 * - Operadores aritméticos: +, -, *, /
 * - Parênteses: (, )
 * - Chaves: {, }
 * - Atribuição: :=
 * - Strings: "[^"]*"
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
    ArithOp,
    Assign,
    Paren,
    StrLit,
    Comma,
    Bracket,
    Semicolon,
}

#[derive(Debug)]
enum TokenValue {
    Lexeme(String),
    Number(i32),
    RelopLabel(String),
    ArithOpLabel(String),
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

    // Define keywords hardcoded
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

    let mut symbol_table: HashSet<String> = keywords.clone(); // Inclui keywords na tabela de símbolos

    match tokenize(&contents, &keywords, &mut symbol_table) {
        Ok(tokens) => {
            // Saída do programa
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

fn tokenize(
    input: &str,
    keywords: &HashSet<String>,
    symbol_table: &mut HashSet<String>,
) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    while let Some(&ch) = chars.peek() {
        // Ignora espaços em branco e novas linhas
        if ch.is_whitespace() {
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

        // Tenta identificar diferentes tokens
        let token_parsers: Vec<
            fn(&mut std::iter::Peekable<std::str::Chars>, &HashSet<String>) -> Option<Token>,
        > = vec![
            parse_identifier,
            parse_number,
            parse_relop,
            parse_arith_op,
            parse_paren,
            parse_assignment,
            parse_string_literal,
            parse_comma,
            parse_bracket,
            parse_semicolon,
        ];

        for parser in token_parsers {
            let mut temp_chars = chars.clone();
            if let Some(token) = parser(&mut temp_chars, keywords) {
                if token.lexeme.len() > max_length {
                    max_length = token.lexeme.len();
                    max_token = Some(token);
                }
            }
        }

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
            if let TokenType::Id = token.token_type {
                symbol_table.insert(token.lexeme.clone());
            }
            tokens.push(token);
        } else {
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

fn parse_number(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
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

fn parse_relop(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
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

/*
* Parser de caractere não identificado (wildcard)
* Consome o caractere e retorna uma mensagem de erro
*/
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

fn parse_arith_op(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    if let Some(&ch) = chars.peek() {
        if "+-*/".contains(ch) {
            let lexeme = ch.to_string();
            chars.next();
            return Some(Token {
                token_type: TokenType::ArithOp,
                lexeme: lexeme.clone(),
                value: TokenValue::ArithOpLabel(lexeme),
            });
        }
    }
    None
}

fn parse_paren(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    if let Some(&ch) = chars.peek() {
        if ch == '(' || ch == ')' {
            let lexeme = ch.to_string();
            chars.next();
            return Some(Token {
                token_type: TokenType::Paren,
                lexeme: lexeme.clone(),
                value: TokenValue::Lexeme(lexeme),
            });
        }
    }
    None
}

fn parse_assignment(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    let mut lexeme = String::new();
    let mut state = 0;

    while let Some(&ch) = chars.peek() {
        match state {
            0 => {
                if ch == ':' {
                    lexeme.push(ch);
                    chars.next();
                    state = 1;
                } else {
                    break;
                }
            }
            1 => {
                if ch == '=' {
                    lexeme.push(ch);
                    chars.next();
                    state = 2;
                } else {
                    break;
                }
            }
            2 => break,
            _ => break,
        }
    }

    if lexeme == ":=" {
        Some(Token {
            token_type: TokenType::Assign,
            lexeme: lexeme.clone(),
            value: TokenValue::Lexeme(lexeme),
        })
    } else {
        None
    }
}

fn parse_string_literal(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    let mut lexeme = String::new();
    if let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next(); // Consume opening quote
            while let Some(&ch) = chars.peek() {
                if ch == '"' {
                    chars.next(); // Consume closing quote
                    return Some(Token {
                        token_type: TokenType::StrLit,
                        lexeme: lexeme.clone(),
                        value: TokenValue::Lexeme(lexeme),
                    });
                } else {
                    lexeme.push(ch);
                    chars.next();
                }
            }
        }
    }
    None
}

fn parse_comma(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    if let Some(&ch) = chars.peek() {
        if ch == ',' {
            let lexeme = ch.to_string();
            chars.next();
            return Some(Token {
                token_type: TokenType::Comma,
                lexeme: lexeme.clone(),
                value: TokenValue::Lexeme(lexeme),
            });
        }
    }
    None
}

fn parse_bracket(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    if let Some(&ch) = chars.peek() {
        if ch == '{' || ch == '}' {
            let lexeme = ch.to_string();
            chars.next();
            return Some(Token {
                token_type: TokenType::Bracket,
                lexeme: lexeme.clone(),
                value: TokenValue::Lexeme(lexeme),
            });
        }
    }
    None
}

fn parse_semicolon(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    _: &HashSet<String>,
) -> Option<Token> {
    if let Some(&ch) = chars.peek() {
        if ch == ';' {
            let lexeme = ch.to_string();
            chars.next();
            return Some(Token {
                token_type: TokenType::Semicolon,
                lexeme: lexeme.clone(),
                value: TokenValue::Lexeme(lexeme),
            });
        }
    }
    None
}
