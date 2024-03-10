fn main() {
    let frase = "Hello Bren, I'm here. Hello Steve! Goodbye!";
    let mut total: i32 = 0;

    for palabra in frase.split(" ") {
        if palabra == "Hello" {
            total = total + 1;
        }
    }

    println!("Total: {}", total);
}
