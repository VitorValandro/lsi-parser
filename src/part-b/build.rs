fn main() {
    // Compila o código C gerado pelo Flex
    cc::Build::new()
        .file("lex.yy.c")
        .compile("liblexer.a");
}