use crate::ll1_table::ll1_transition_table;
use crate::token::{Terminal, Token};

pub fn parse(tokens: Vec<Token>) -> Result<(), String> {
    let table = ll1_transition_table();

    let mut stack: Vec<&str> = vec!["$", "MAIN"];
    let mut index = 0;

    while let Some(top) = stack.pop() {
        println!();
        println!("Stack: {:?}", stack);
        println!("Top of stack: {}", top);

        if top == "$" {
            if index == tokens.len() {
                println!("Parsing successful.");
                return Ok(());
            } else {
                println!("Unexpected tokens remaining.");
                return Err("Unexpected tokens remaining.".to_string());
            }
        }

        let current_token = tokens.get(index);

        // Handle end of input
        if current_token.is_none() {
            if let Some(rule) = table.get(&(top, Terminal::DollarSign)) {
                println!("Applying epsilon transition for {}", top);
                if *rule != "ε" {
                    stack.extend(rule.split_whitespace().rev());
                }
                continue;
            } else {
                return Err(format!(
                    "Parsing failed: unexpected remaining symbol `{}` in stack.",
                    top
                ));
            }
        }

        let current_token = current_token.unwrap();
        println!("Current token: {:?}", current_token);

        // Custom handling for `IFSTMT'`
        if top == "IFSTMT'" {
            if current_token.terminal == Terminal::Else {
                println!("Handling `else` branch for IFSTMT'");
                stack.extend(vec!["else", "STMT"].iter().rev());
            } else {
                println!("Applying epsilon transition for IFSTMT' (no else branch)");
            }
            continue;
        }

        // Custom handling for `ATRIBST'`
        if top == "ATRIBST'" {
            if let Some(next_token) = tokens.get(index + 1) {
                if next_token.terminal == Terminal::LeftParen {
                    stack.extend(vec!["FCALL"].iter().rev());
                } else {
                    stack.extend(vec!["EXPR"].iter().rev());
                }
            } else {
                return Err("Unexpected end of input while resolving ATRIBST'.".to_string());
            }
            continue;
        }

        if let Some(rule) = table.get(&(top, current_token.terminal.clone())) {
            println!("Applying rule: {} -> {}", top, rule);

            if *rule != "ε" {
                stack.extend(rule.split_whitespace().rev());
            }
        } else if top == current_token.terminal.as_str() {
            println!("Matched terminal: {}", top);
            index += 1;
        } else {
            println!(
                "Syntax error: expected `{}`, found `{}`.",
                top,
                current_token.terminal.as_str()
            );
            return Err(format!(
                "Syntax error: expected `{}`, found `{}`.",
                top,
                current_token.terminal.as_str()
            ));
        }
    }

    println!("Parsing completed.");
    Ok(())
}
