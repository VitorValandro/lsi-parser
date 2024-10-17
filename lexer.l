%{
#include <stdio.h>
%}

%option noyywrap

%%

"int"            { printf("KEYWORD_INT\n"); }
"if"             { printf("KEYWORD_IF\n"); }
"else"           { printf("KEYWORD_ELSE\n"); }
"def"            { printf("KEYWORD_DEF\n"); }
"print"          { printf("KEYWORD_PRINT\n"); }
"return"         { printf("KEYWORD_RETURN\n"); }

[a-zA-Z_][a-zA-Z0-9_]*  { printf("IDENTIFIER: %s\n", yytext); }
[0-9]+                  { printf("NUMBER: %s\n", yytext); }

"<="              { printf("REL_OP_LE\n"); }
">="              { printf("REL_OP_GE\n"); }
"=="              { printf("REL_OP_EQ\n"); }
"<>"              { printf("REL_OP_NE\n"); }
"<"               { printf("REL_OP_LT\n"); }
">"               { printf("REL_OP_GT\n"); }

"+"               { printf("PLUS\n"); }
"-"               { printf("MINUS\n"); }
"*"               { printf("MULT\n"); }
"/"               { printf("DIV\n"); }

":="              { printf("ASSIGN\n"); }

"("               { printf("LPAREN\n"); }
")"               { printf("RPAREN\n"); }
"{"               { printf("LBRACE\n"); }
"}"               { printf("RBRACE\n"); }
","               { printf("COMMA\n"); }
";"               { printf("SEMI\n"); }

[ \t\n]+          { /* Ignora espaços, tabs e novas linhas */ }
.                 { printf("UNKNOWN: %s\n", yytext); }

%%

int main() {
    yylex();  // Chama o lexer para processar a entrada
    return 0;
}
