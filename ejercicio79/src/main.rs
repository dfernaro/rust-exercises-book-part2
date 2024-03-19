fn uppercase(palabra: String) -> String {
    let resultado: String = palabra.to_ascii_uppercase();
    resultado
}

fn main() {
    println!(
        "Antes: {}, Después: {}",
        String::from("Manzana"),
        uppercase(String::from("Manzana"))
    );
    println!(
        "Antes: {}, Después: {}",
        String::from("Pera"),
        uppercase(String::from("Pera"))
    );
}
