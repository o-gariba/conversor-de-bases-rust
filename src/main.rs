use std::io::{self, Write};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use colored::*;

fn main() {
    loop {
        print!("[2J[1;1H");
        println!("{}", "=========================================".cyan().bold());
        println!("{}", "      CONVERSOR DE BASES NUMÉRICAS".cyan().bold());
        println!("{}", "=========================================".cyan().bold());

        print!("{}", "[?] Digite a base de ORIGEM (2 a 16): ".yellow());
        io::stdout().flush().unwrap();
        let mut base_origem_str = String::new();
        io::stdin()
            .read_line(&mut base_origem_str)
            .expect("Falha ao ler entrada");

        let base_origem: u32 = match base_origem_str.trim().parse() {
            Ok(base) if (2..=16).contains(&base) => base,
            _ => {
                    println!("{}", "[!] BASE INVÁLIDA! Pressione Enter para tentar novamente.".red());
                    io::stdin().read_line(&mut String::new()).unwrap();
                    continue;
            }
        };

        print!("{}", "[?] Digite o valor de ORIGEM a ser convertido: ".yellow());
        io::stdout().flush().unwrap();
        let mut valor_entrada = String::new();
        io::stdin()
            .read_line(&mut valor_entrada)
            .expect("Falha ao ler valor de entrada");

        let mut valor_valido = true;
        let mut valor_decimal: BigUint = BigUint::ZERO;

        for c in valor_entrada.trim().chars() {
            match c.to_digit(base_origem) {
                Some(digito) => {
                    valor_decimal = valor_decimal * BigUint::from(base_origem) + BigUint::from(digito);
                }
                None => {
                    println!(
                        "{}", 
                        format!(
                            "[!] DÍGITO INVÁLIDO '{}' PARA A BASE '{}'. Pressione Enter para tentar novamente.", 
                            c, 
                            base_origem
                        ).red()
                    );
                    io::stdin()
                        .read_line(&mut String::new())
                        .unwrap();
                    valor_valido = false;
                    break;
                }
            }
        }

        if !valor_valido {
            continue;
        }

        print!("{}", "[?] Digite a base de DESTINO (2 a 16): ".yellow());
        io::stdout().flush().unwrap();
        let mut base_destino_str = String::new();
        io::stdin()
            .read_line(&mut base_destino_str)
            .expect("Falha ao ler entrada");

        let base_destino: u32 = match base_destino_str.trim().parse() {
            Ok(base) if (2..=16).contains(&base) => base,
            _ => {
                println!(
                    "{}",
                    "[!] BASE INVÁLIDA! Pressione Enter para tentar novamente.".red()
                );
                io::stdin().read_line(&mut String::new()).unwrap();
                continue;
            }
        };

        let mut valor_final_inverso = Vec::new();

        if valor_decimal == BigUint::ZERO {
            valor_final_inverso.push('0');
        } else {

            // Aqui, valor_decimal pode ser entendido como valor_resto também
            while valor_decimal > BigUint::ZERO {
                let resto = &valor_decimal % BigUint::from(base_destino);

                // Essa conversão sempre será válida pois estamos trabalhando com bases pequenas,
                // apenas
                match resto.to_u32() {
                    Some(r) => match std::char::from_digit(r, base_destino) {
                        Some(digito) => valor_final_inverso.push(digito.to_ascii_uppercase()),
                        None => break, 
                    },
                    None => break,
                }
                valor_decimal /= BigUint::from(base_destino);
            }
        }

        valor_final_inverso.reverse();
        // percorro todo o vetor coletando cada indice como uma string a ser concatenada ao
        // valor_vinal
        let valor_final: String = valor_final_inverso.iter().collect();

        println!(
                "{}",
                "-----------------------------------------".green()
        );
        println!(
            "{}{}",
            "[✓] RESULTADO: ".bold().green(),
            format!(
                "O valor '{}' (base {}) é igual a '{}' (base {}).",
                valor_entrada.trim(),
                base_origem,
                valor_final,
                base_destino
            ).green()
        );
        println!(
            "{}", 
            "-----------------------------------------".green()
        );

        print!(
            "{}", 
            "[?] Deseja fazer outra conversão? (s/n): ".yellow()
        );
        io::stdout().flush().unwrap();
        let mut resposta = String::new();
        io::stdin()
            .read_line(&mut resposta)
            .expect("Falha ao ler resposta");

        if resposta.trim().to_lowercase() != "s" {
            break;
        }
    }

    println!(
        "{}", 
        "Obrigado por usar o conversor! 👋".cyan()
    );
}
