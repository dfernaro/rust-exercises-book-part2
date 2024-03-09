fn main() {
    let frase = "Me gusta programar";

    for palabra in frase.split_whitespace() {
        println!("{}", palabra);
    }
}
