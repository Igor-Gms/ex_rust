pub fn abbreviate(phrase: &str) -> String {
    let mut sigla = String::new();
    let mut inicio_palavra = true;
    let mut anterior_minusculo = false;

    for caractere in phrase.chars() {
        if caractere.is_whitespace() || caractere == '-' || caractere == '_' {
            inicio_palavra = true;
            anterior_minusculo = false;
            continue;
        }

        if !caractere.is_alphabetic() {
            continue;
        }
        
        if inicio_palavra || (anterior_minusculo && caractere.is_uppercase()) {
            sigla.extend(caractere.to_uppercase());
        }

        inicio_palavra = false;
        anterior_minusculo = caractere.is_lowercase();
    }

    sigla
}
