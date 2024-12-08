/// Realiza a análise sintática de uma sequência de tokens utilizando uma tabela de transição LL(1).
///
/// # Parâmetros
/// - `tokens`: Um vetor de tokens a serem analisados.
///
/// # Retorna
/// - `Ok(String)`: Uma mensagem indicando que a análise sintática foi bem-sucedida, juntamente com a sequência de correspondências.
/// - `Err(String)`: Uma mensagem de erro indicando que ocorreu um erro sintático.
///
/// # Erros
/// - Retorna um erro se houver tokens inesperados restantes após o processamento.
/// - Retorna um erro se um não terminal não puder ser resolvido.
use crate::ll1_table::ll1_transition_table;
use crate::token::{Terminal, Token};

pub fn parse(tokens: Vec<Token>) -> Result<String, String> {
    let table = ll1_transition_table();

    let mut stack: Vec<&str> = vec!["$", "MAIN"];
    let mut index = 0;
    let mut matches: Vec<String> = vec![]; // Para armazenar a sequência de "matches"

    while let Some(top) = stack.pop() {
        println!();
        println!("Pilha: {:?}", stack);
        println!("Topo da pilha: {}", top);

        if top == "$" {
            if index == tokens.len() {
                // Sucesso: retorna a sequência de "matches"
                let match_sequence = matches.join(" / ");
                return Ok(format!(
                    "\nAnálise sintática bem-sucedida.\nSequência de correspondências: \n[{}].",
                    match_sequence
                ));
            } else {
                return Err("Tokens inesperados restantes.".to_string());
            }
        }

        let current_token = tokens.get(index);

        // Verifica se o final da entrada foi alcançado
        if current_token.is_none() {
            if let Some(rule) = table.get(&(top, Terminal::DollarSign)) {
                println!("Aplicando transição epsilon para {}", top);
                if *rule != "ε" {
                    stack.extend(rule.split_whitespace().rev());
                }
                continue;
            } else {
                return Err(format!(
                    "Erro sintático: não terminal `{}` restante na pilha.",
                    top
                ));
            }
        }

        let current_token = current_token.unwrap();
        println!("Token atual: {:?}", current_token);

        // Casos especiais para `IFSTMT'`
        if top == "IFSTMT'" {
            if current_token.terminal == Terminal::Else {
                println!("Tratando ramo `else` para IFSTMT'");
                stack.extend(vec!["else", "STMT"].iter().rev());
            } else {
                println!("Aplicando transição epsilon para IFSTMT' (sem ramo else)");
            }
            continue;
        }

        // Casos especiais para `ATRIBST'`
        if top == "ATRIBST'" {
            if let Some(next_token) = tokens.get(index + 1) {
                if next_token.terminal == Terminal::LeftParen {
                    println!("Tratando chamada de função para ATRIBST'");
                    stack.extend(vec!["FCALL"].iter().rev());
                } else {
                    println!("Tratando expressão para ATRIBST'");
                    stack.extend(vec!["EXPR"].iter().rev());
                }
            } else {
                return Err("Fim inesperado da entrada ao resolver ATRIBST'.".to_string());
            }
            continue;
        }

        if let Some(rule) = table.get(&(top, current_token.terminal.clone())) {
            println!("Aplicando regra: {} -> {}", top, rule);

            if *rule != "ε" {
                stack.extend(rule.split_whitespace().rev());
            }
        } else if top == current_token.terminal.as_str() {
            // Casamento bem-sucedido
            println!("Terminal correspondente: {}", top);
            matches.push(top.to_string());
            index += 1;
        } else {
            // Erro sintático
            if current_token.terminal.as_str() != top {
                return Err(format!(
                    "Erro sintático: esperado `{}`, encontrado `{}` no token {}.",
                    top,
                    current_token.terminal.as_str(),
                    index + 1
                ));
            } else {
                return Err(format!(
                    "Erro sintático: não terminal `{}` não encontrado na tabela de análise.",
                    top
                ));
            }
        }
    }

    println!("Análise sintática concluída.");
    Ok("Análise sintática concluída.".to_string())
}
