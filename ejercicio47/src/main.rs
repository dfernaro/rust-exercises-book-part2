fn main() {
    let frase = "Mi nombre es David";
    let mut total: i32 = 0;

    for letra in frase.chars() {
        if letra.to_ascii_lowercase() == 'a'
            || letra.to_ascii_lowercase() == 'e'
            || letra.to_ascii_lowercase() == 'i'
            || letra.to_ascii_lowercase() == 'o'
            || letra.to_ascii_lowercase() == 'u'
        {
            total = total + 1;
        }
    }

    println!("Número de vocales en el texto: {}", total);
}
