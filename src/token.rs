#[derive(Debug)]
/// Esse arquivo contém os tipos, structs e implementações usadas para representar tokens e terminais.
pub enum TokenType {
    Id,
    Int,
    Relop,
    Keyword,
    ArithOp,
    Assign,
    Paren,
    Comma,
    Bracket,
    Semicolon,
}

/// Enum para tipos de token que refletem os requisitos da gramática.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Terminal {
    Int,
    Id,
    Print,
    Return,
    If,
    Def,
    Plus,         // +
    Minus,        // -
    Multiply,     // *
    Divide,       // /
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    Comma,        // ,
    Semicolon,    // ;
    Num,          // num
    Equals,       // :=
    LessThan,     // <
    LessEqual,    // <=
    GreaterThan,  // >
    GreaterEqual, // >=
    Equal,        // ==
    NotEqual,     // <>
    Else,         // else
    DollarSign,   // $
    Epsilon,      // ε
}

impl Terminal {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Terminal::Int => "int",
            Terminal::Id => "id",
            Terminal::Print => "print",
            Terminal::Return => "return",
            Terminal::If => "if",
            Terminal::Def => "def",
            Terminal::LeftBrace => "{",
            Terminal::RightBrace => "}",
            Terminal::LeftParen => "(",
            Terminal::RightParen => ")",
            Terminal::Comma => ",",
            Terminal::Semicolon => ";",
            Terminal::Num => "num",
            Terminal::Equals => ":=",
            Terminal::LessThan => "<",
            Terminal::LessEqual => "<=",
            Terminal::GreaterThan => ">",
            Terminal::GreaterEqual => ">=",
            Terminal::Equal => "==",
            Terminal::NotEqual => "<>",
            Terminal::Else => "else",
            Terminal::DollarSign => "$",
            Terminal::Epsilon => "ε",
            Terminal::Plus => "+",
            Terminal::Minus => "-",
            Terminal::Multiply => "*",
            Terminal::Divide => "/",
        }
    }

    pub fn from_str(s: &str) -> Option<Terminal> {
        match s {
            "int" => Some(Terminal::Int),
            "id" => Some(Terminal::Id),
            "print" => Some(Terminal::Print),
            "return" => Some(Terminal::Return),
            "if" => Some(Terminal::If),
            "def" => Some(Terminal::Def),
            "{" => Some(Terminal::LeftBrace),
            "}" => Some(Terminal::RightBrace),
            "(" => Some(Terminal::LeftParen),
            ")" => Some(Terminal::RightParen),
            "," => Some(Terminal::Comma),
            ";" => Some(Terminal::Semicolon),
            "num" => Some(Terminal::Num),
            ":=" => Some(Terminal::Equals),
            "<" => Some(Terminal::LessThan),
            "<=" => Some(Terminal::LessEqual),
            ">" => Some(Terminal::GreaterThan),
            ">=" => Some(Terminal::GreaterEqual),
            "==" => Some(Terminal::Equal),
            "<>" => Some(Terminal::NotEqual),
            "else" => Some(Terminal::Else),
            "$" => Some(Terminal::DollarSign),
            "ε" => Some(Terminal::Epsilon),
            "+" => Some(Terminal::Plus),
            "-" => Some(Terminal::Minus),
            "*" => Some(Terminal::Multiply),
            "/" => Some(Terminal::Divide),
            _ => None,
        }
    }
}

/// Enum para valores de token com variantes para diferentes tipos de dados.
#[derive(Debug)]
pub enum TokenValue {
    Lexeme(String),       // Lexema geral (e.g. nome de variável)
    Number(i32),          // Valor numérico
    RelopLabel(String),   // Operadores relacionais (e.g., <, >)
    ArithOpLabel(String), // Operadores aritméticos (e.g., +, -)
}

/// Struct para tokens usados durante a análise.
/// Tokens são gerados na análise léxica e repassados para a análise sintática
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType, // O tipo de token
    pub lexeme: String,        // O lexema associado (a string literal)
    pub value: TokenValue,     // O valor associado (se houver)
    pub terminal: Terminal,    // O símbolo terminal que será usado na análise sintática
}
