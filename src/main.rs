use std::io;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

fn main() {
    loop{
        println!("\n\n----------- CONVERSOR DE BASES -----------\n\n");
        println!("Digite a base de ORIGEM (2 até 16)");

       let mut base_origem = String::new();
       io::stdin()
           .read_line(&mut base_origem)
           .expect("Falha ao ler base de entrada");

        let base_origem: u32 = match base_origem.trim().parse() {
            Ok(base) => base,
            Err(_) => continue,
        };
        
        if !(base_origem >= 2 && base_origem <= 16) {
            println!("\nBASE INVÁLIDA!\n");
            continue;
        };

       println!("\nDigite o valor de ORIGEM a ser convertido para a nova base");
       let mut valor_entrada = String::new();
       io::stdin()
           .read_line(&mut valor_entrada)
           .expect("Falha ao ler valor de entrada");

        let mut valor_valido = true;
        let mut valor_decimal: BigUint = BigUint::ZERO;

        for c in valor_entrada.trim().chars() {
            match c.to_digit(base_origem) {
                Some(digito) => {
                    valor_decimal = valor_decimal * BigUint::from(base_origem) + BigUint::from(digito)
                },
                None => {
                    println!("\nDÍGITO INVÁLIDO '{c}' ENCONTRADO PARA A BASE '{base_origem}'\n");
                    valor_valido = false;
                    break;
                }
            } 
        };

        if !valor_valido {
            continue;
        };


       println!("Digite a base de DESTINO do valor a ser inserido.\nBases aceitas: 2, 3, 4, ..., 15, 16");
       let mut base_destino = String::new();
       io::stdin()
           .read_line(&mut base_destino)
           .expect("Falha ao ler base de entrada");

        let base_destino: u32 = match base_destino.trim().parse() {
            Ok(base) => base,
            Err(_) => continue,
        };
        
        if !(base_destino >= 2 && base_destino <= 16) {
            println!("\nBASE INVÁLIDA!\n");
            continue;
        };

        let mut valor_final_inverso = Vec::new();

        while valor_decimal != BigUint::ZERO {
            let resto = &valor_decimal % BigUint::from(base_destino);

            match resto.to_u32() {
                Some(resto) => match std::char::from_digit(resto,base_destino) {
                    Some(digito) => valor_final_inverso.push(digito.to_ascii_uppercase()),
                    None => break,
                },
                None => break,
            }
            valor_decimal /= BigUint::from(base_destino);
        }

        valor_final_inverso.reverse();

        let valor_final: String = valor_final_inverso.iter().collect();

        println!("\nRESULTADO: o valor {valor_entrada} na base {base_origem} é o mesmo que {valor_final} na base {base_destino}.\n");
    }
}
