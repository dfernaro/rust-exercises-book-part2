fn palabra_reves(palabra: String) -> String {
    let mut resultado: String = String::new();

    for letra in palabra.chars().rev() {
        resultado.push(letra);
    }

    resultado
}

fn main() {
    let palabra: String = String::from("Testing");
    println!("Antes: {}", palabra);
    println!("Después: {}", palabra_reves(palabra));
}
