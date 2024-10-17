# Analisador Léxico Simples

Este projeto é um analisador léxico simples para identificadores, números inteiros e operadores relacionais. Ele foi desenvolvido como parte de um trabalho acadêmico e é capaz de reconhecer tokens baseados nas seguintes expressões regulares:

Identificador: `[a-zA-Z][a-zA-Z0-9]\*`
Número inteiro: `[0-9]+`
Operadores relacionais: `==`, `!=`, `>=`, `<=`, `>`, `<`

#### Autores

- Vitor Matheus Valandro da Rosa (22102567)
- Pedro Henrique Nascimento Rocha (22100918)

Pré-requisitos
[Rust](https://www.rust-lang.org/tools/install)

## Execução

Para executar o analisador léxico, utilize o comando abaixo dentro do diretório do projeto, fornecendo o caminho para o arquivo de entrada como argumento:

```sh
cargo run entrada-A.txt
```

Para compilar o projeto para um executável, execute o comando abaixo:

```sh
cargo build --relase
```

O executável será gerado em `target/release/lexer-simples`.
Para executar o executável, utilize o comando abaixo:

```sh
./target/release/lexer-simples entrada-A.txt
```

## Exemplo de saída

Conforme a especificação do trabalho, o analisador léxico imprime na saída padrão a lista de tokens reconhecidos e a tabela de símbolos.

O projeto tem dois arquivos de entrada prontos, `entrada-A.txt` e `entrada-A-invalida.txt`, que podem ser usados para testar o analisador léxico. O primeiro contém uma lista de tokens válidos e o segundo contém uma lista de tokens inválidos. Abaixo está uma amostra de saída para o arquivo diferentes entradas:

### Entradas Válidas

O arquivo de entrada é o seguinte:

```
identificador123
return
1<3
456
identificador123
if
```

A saída gerada pelo analisador léxico é a seguinte:

```
Lista de Tokens:
Token { token_type: Id, lexeme: "identificador123", value: Lexeme("identificador123") }
Token { token_type: Keyword, lexeme: "return", value: Lexeme("return") }
Token { token_type: Int, lexeme: "1", value: Number(1) }
Token { token_type: Relop, lexeme: "<", value: RelopLabel("LT") }
Token { token_type: Int, lexeme: "3", value: Number(3) }
Token { token_type: Int, lexeme: "456", value: Number(456) }
Token { token_type: Id, lexeme: "identificador123", value: Lexeme("identificador123") }
Token { token_type: Keyword, lexeme: "if", value: Lexeme("if") }

Tabela de Símbolos:
int
float
identificador123
if
else
while
return
char
void
```

A lista de tokens mostra o tipo de cada token, o lexema correspondente e o valor associado ao token. A tabela de símbolos mostra os identificadores reconhecidos pelo analisador léxico. Repare que mesmo que o identificador `identificador123` apareça duas vezes no arquivo de entrada, ele é listado apenas uma vez na tabela de símbolos.

Os valores das listas de token variam de acordo com o tipo do token. Para identificadores, o valor é o próprio lexema. Para números inteiros, o valor é o número inteiro correspondente. Para operadores relacionais, o valor é um enum que representa o tipo de operador relacional.

### Entradas Inválidas

O analisador léxico é capaz de identificar erros léxicos, como identificadores inválidos e números inteiros mal formados. O analisador léxico só consegue analisar um erro por execução, falhando na primeira ocorrência. Abaixo estão exemplos de lexemas que causam falha no analisador léxico:

```
123identificador
_identificador
#constante
12.5
3.14159
identificador_especial
1/2
```

Se entrarmos com a seguinte entrada:

```
identificador123
return
3.14159
456
identificador123
if
```

A saída gerada pelo analisador léxico é a seguinte:

```
Erro: Caractere não identificado '.' na linha 3, coluna 2
```

Porque "3.14159" não é um número inteiro válido reconhecido pelo analisador léxico.
