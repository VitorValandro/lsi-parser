use std::collections::HashMap;

use crate::token::Terminal;

pub fn ll1_transition_table() -> HashMap<(&'static str, Terminal), &'static str> {
    let mut table = HashMap::new();

    // MAIN
    table.insert(("MAIN", Terminal::Int), "STMT");
    table.insert(("MAIN", Terminal::Id), "STMT");
    table.insert(("MAIN", Terminal::Print), "STMT");
    table.insert(("MAIN", Terminal::Return), "STMT");
    table.insert(("MAIN", Terminal::If), "STMT");
    table.insert(("MAIN", Terminal::LeftBrace), "STMT");
    table.insert(("MAIN", Terminal::Def), "FLIST");
    table.insert(("MAIN", Terminal::Semicolon), "STMT");
    table.insert(("MAIN", Terminal::DollarSign), "ε");

    // FLIST
    table.insert(("FLIST", Terminal::Def), "FDEF FLIST'");
    table.insert(("FLIST", Terminal::DollarSign), "ε");

    // FLIST'
    table.insert(("FLIST'", Terminal::Def), "FDEF FLIST'");
    table.insert(("FLIST'", Terminal::DollarSign), "ε");

    // FDEF
    table.insert(("FDEF", Terminal::Def), "def id ( PARLIST ) { STMTLIST }");

    // PARLIST
    table.insert(("PARLIST", Terminal::Int), "int id PARLIST'");
    table.insert(("PARLIST", Terminal::RightParen), "ε");

    // PARLIST'
    table.insert(("PARLIST'", Terminal::Comma), ", PARLIST");
    table.insert(("PARLIST'", Terminal::RightParen), "ε");

    // VARLIST
    table.insert(("VARLIST", Terminal::Id), "id VARLIST'");

    // VARLIST'
    table.insert(("VARLIST'", Terminal::Comma), ", VARLIST");
    table.insert(("VARLIST'", Terminal::Semicolon), "ε");

    // STMT
    table.insert(("STMT", Terminal::Int), "int VARLIST ;");
    table.insert(("STMT", Terminal::Id), "ATRIBST ;");
    table.insert(("STMT", Terminal::Print), "PRINTST ;");
    table.insert(("STMT", Terminal::Return), "RETURNST ;");
    table.insert(("STMT", Terminal::If), "IFSTMT");
    table.insert(("STMT", Terminal::LeftBrace), "{ STMTLIST }");
    table.insert(("STMT", Terminal::Semicolon), ";");

    // ATRIBST
    table.insert(("ATRIBST", Terminal::Id), "id := ATRIBST'");

    // ATRIBST'
    table.insert(("ATRIBST'", Terminal::Num), "EXPR");
    table.insert(("ATRIBST'", Terminal::LeftParen), "EXPR");
    table.insert(("ATRIBST'", Terminal::Id), "FCALL");

    // FCALL
    table.insert(("FCALL", Terminal::Id), "id ( PARLISTCALL )");

    // PARLISTCALL
    table.insert(("PARLISTCALL", Terminal::Id), "id PARLISTCALL'");
    table.insert(("PARLISTCALL", Terminal::RightParen), "ε");

    // PARLISTCALL'
    table.insert(("PARLISTCALL'", Terminal::Comma), ", PARLISTCALL");
    table.insert(("PARLISTCALL'", Terminal::RightParen), "ε");

    // PRINTST
    table.insert(("PRINTST", Terminal::Print), "print EXPR");

    // RETURNST
    table.insert(("RETURNST", Terminal::Return), "return RETURNST'");

    // RETURNST'
    table.insert(("RETURNST'", Terminal::Id), "id");
    table.insert(("RETURNST'", Terminal::Semicolon), "ε");

    // IFSTMT
    table.insert(("IFSTMT", Terminal::If), "if ( EXPR ) STMT IFSTMT'");

    // IFSTMT'
    table.insert(("IFSTMT'", Terminal::Else), "else STMT");
    table.insert(("IFSTMT'", Terminal::Int), "ε");
    table.insert(("IFSTMT'", Terminal::Id), "ε");
    table.insert(("IFSTMT'", Terminal::Print), "ε");
    table.insert(("IFSTMT'", Terminal::Return), "ε");
    table.insert(("IFSTMT'", Terminal::If), "ε");
    table.insert(("IFSTMT'", Terminal::LeftBrace), "ε");
    table.insert(("IFSTMT'", Terminal::Semicolon), "ε");
    table.insert(("IFSTMT'", Terminal::DollarSign), "ε");

    // STMTLIST
    table.insert(("STMTLIST", Terminal::Int), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::Id), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::Print), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::Return), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::If), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::LeftBrace), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::Semicolon), "STMT STMTLIST'");
    table.insert(("STMTLIST", Terminal::RightBrace), "ε");

    // STMTLIST'
    table.insert(("STMTLIST'", Terminal::Int), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::Id), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::Print), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::Return), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::If), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::LeftBrace), "STMTLIST");
    table.insert(("STMTLIST'", Terminal::RightBrace), "ε");

    // EXPR
    table.insert(("EXPR", Terminal::Num), "NUMEXPR EXPR'");
    table.insert(("EXPR", Terminal::LeftParen), "NUMEXPR EXPR'");
    table.insert(("EXPR", Terminal::Id), "NUMEXPR EXPR'");

    // EXPR'
    table.insert(("EXPR'", Terminal::LessThan), "< NUMEXPR");
    table.insert(("EXPR'", Terminal::LessEqual), "<= NUMEXPR");
    table.insert(("EXPR'", Terminal::GreaterThan), "> NUMEXPR");
    table.insert(("EXPR'", Terminal::GreaterEqual), ">= NUMEXPR");
    table.insert(("EXPR'", Terminal::Equal), "== NUMEXPR");
    table.insert(("EXPR'", Terminal::NotEqual), "<> NUMEXPR");
    table.insert(("EXPR'", Terminal::Semicolon), "ε");

    // NUMEXPR
    table.insert(("NUMEXPR", Terminal::Num), "TERM NUMEXPR'");
    table.insert(("NUMEXPR", Terminal::LeftParen), "TERM NUMEXPR'");
    table.insert(("NUMEXPR", Terminal::Id), "TERM NUMEXPR'");

    // NUMEXPR'
    table.insert(("NUMEXPR'", Terminal::Plus), "+ TERM NUMEXPR'");
    table.insert(("NUMEXPR'", Terminal::Minus), "- TERM NUMEXPR'");
    table.insert(("NUMEXPR'", Terminal::LessThan), "ε");
    table.insert(("NUMEXPR'", Terminal::LessEqual), "ε");
    table.insert(("NUMEXPR'", Terminal::GreaterThan), "ε");
    table.insert(("NUMEXPR'", Terminal::GreaterEqual), "ε");
    table.insert(("NUMEXPR'", Terminal::Equal), "ε");
    table.insert(("NUMEXPR'", Terminal::NotEqual), "ε");
    table.insert(("NUMEXPR'", Terminal::Semicolon), "ε");

    // TERM
    table.insert(("TERM", Terminal::Num), "FACT TERM'");
    table.insert(("TERM", Terminal::LeftParen), "FACT TERM'");
    table.insert(("TERM", Terminal::Id), "FACT TERM'");

    // TERM'
    table.insert(("TERM'", Terminal::Multiply), "* FACT TERM'");
    table.insert(("TERM'", Terminal::Divide), "/ FACT TERM'");
    table.insert(("TERM'", Terminal::Plus), "ε");
    table.insert(("TERM'", Terminal::Minus), "ε");
    table.insert(("TERM'", Terminal::LessThan), "ε");
    table.insert(("TERM'", Terminal::LessEqual), "ε");
    table.insert(("TERM'", Terminal::GreaterThan), "ε");
    table.insert(("TERM'", Terminal::GreaterEqual), "ε");
    table.insert(("TERM'", Terminal::Equal), "ε");
    table.insert(("TERM'", Terminal::NotEqual), "ε");
    table.insert(("TERM'", Terminal::Semicolon), "ε");

    // FACT
    table.insert(("FACT", Terminal::Num), "num");
    table.insert(("FACT", Terminal::LeftParen), "( EXPR )");
    table.insert(("FACT", Terminal::Id), "id");

    table
}
