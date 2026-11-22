pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn main() -> std::io::Result<()> {
    let mut entrada = String::new();

    println!("Digite um ano:");
    std::io::stdin().read_line(&mut entrada)?;

    let ano: u64 = match entrada.trim().parse() {
        Ok(valor) => valor,
        Err(_) => {
            eprintln!("Ano inválido. Digite um número inteiro não negativo.");
            return Ok(());
        }
    };

    if is_leap_year(ano) {
        println!("{ano} é um ano bissexto.");
    } else {
        println!("{ano} não é um ano bissexto.");
    }

    Ok(())
}
