fn contar_vocales(frase: String) -> i32 {
    let mut count: i32 = 0;

    for letra in frase.chars() {
        if letra.to_ascii_lowercase() == 'a'
            || letra.to_ascii_lowercase() == 'e'
            || letra.to_ascii_lowercase() == 'i'
            || letra.to_ascii_lowercase() == 'o'
            || letra.to_ascii_lowercase() == 'u'
        {
            count = count + 1;
        }
    }

    count
}

fn main() {
    let frase = String::from("Esto ES un ejemplo");
    println!("La frase tiene {} vocales", contar_vocales(frase));
}
