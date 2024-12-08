# Parser Preditivo

Este projeto é um parser preditivo para a linguage LSI-2024-2. Ele foi desenvolvido como parte de um trabalho acadêmico e é capaz de receber um arquivo de entrada contendo um código fonte escrito na linguagem LSI-2024-2, fazer a análise léxica do código e o parsing sintático do código, detectando erros léxicos e sintáticos na entrada.

#### Autores

- Vitor Matheus Valandro da Rosa (22102567)
- Pedro Henrique Nascimento Rocha (22100918)

Pré-requisitos
[Rust](https://www.rust-lang.org/tools/install)

## Execução

Para executar o parser, utilize o comando abaixo dentro do diretório do projeto, fornecendo o caminho para o arquivo de entrada como argumento:

```sh
cargo run entrada-valida-1.lsi
```

Para compilar o projeto para um executável, execute o comando abaixo:

```sh
cargo build --release
```

O executável será gerado em `target/release/lexer-simples`.
Para executar o executável, utilize o comando abaixo:

```sh
./target/release/parser entrada-valida-1.lsi
```

## A Análise Léxica

O analisador léxico implementado é uma adaptação do analisador léxico desenvolvido no trabalho anterior. Ele é capaz de reconhecer tokens da linguagem LSI-2024-2, como identificadores, números inteiros, operadores aritméticos, operadores relacionais e palavras reservadas. O analisar gera uma lista sequencial dos tokens reconhecidos e gera uma tabela de símbolos. Os tokens processados são usados na etapa posterior de análise sintática. O analisador léxico é capaz de detectar erros léxicos na entrada, como caracteres inválidos e números mal formados.

## A Análise Sintática

O analisador sintático implementado é um parser preditivo para a linguagem LSI-2024-2. Ele é capaz de reconhecer a estrutura sintática da linguagem, detectando erros sintáticos na entrada. O parser é implementado com base em uma pilha e uma tabela de análise sintática (LL1). A tabela de análise sintática é gerada a partir da gramática da linguagem LSI-2024-2. Para cada token recebido da análise léxica, o parser consulta a tabela de análise sintática para decidir qual regra de produção aplicar. O parser é capaz de detectar erros sintáticos na entrada, como tokens inesperados e tokens faltantes.
