fn main() {
    let frase = "Esto es un ejemplo";
    let mut total: i32 = 0;

    for palabra in frase.split(" ") {
        total = total + 1;
    }

    println!("Total palabras: {}", total);
}
