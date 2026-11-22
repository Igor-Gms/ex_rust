pub fn abbreviate(phrase: &str) -> String {
    let mut acronimo = String::new();

    for palavra in phrase.split(|c: char| c.is_whitespace() || c == '-' || c == '_') {
        let mut inicio_palavra = true;
        let mut anterior_minusculo = false;

        for letra in palavra.chars() {
            if !letra.is_alphabetic() {
                continue;
            }

            if inicio_palavra || (anterior_minusculo && letra.is_uppercase()) {
                acronimo.extend(letra.to_uppercase());
            }

            inicio_palavra = false;
            anterior_minusculo = letra.is_lowercase();
        }
    }

    acronimo
}

// Entrada do programa para executar no computador pelo terminal.
fn main() -> std::io::Result<()> {
    let mut entrada = String::new();

    println!("Digite uma frase:");
    std::io::stdin().read_line(&mut entrada)?;

    println!("Acrônimo: {}", abbreviate(&entrada));
    Ok(())
}
