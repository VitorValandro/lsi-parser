extern "C" {
    fn yylex();
    fn yyin(); // Arquivo de entrada
}

use std::ffi::CString;
use std::ptr;

fn main() {
    // Abre o arquivo de entrada
    let input_file = CString::new("entrada-B.txt").unwrap();
    unsafe {
        // Define o arquivo de entrada para o analisador
        yyin = fopen(input_file.as_ptr(), CString::new("r").unwrap().as_ptr());
        
        // Chama o analisador léxico gerado pelo Flex
        yylex();

        // Fecha o arquivo de entrada
        fclose(yyin);
    }
}
