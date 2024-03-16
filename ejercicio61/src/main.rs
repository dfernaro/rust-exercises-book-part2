fn main() {
    let lista1 = [
        String::from("manzana"),
        String::from("cereza"),
        String::from("kiwi"),
        String::from("mango"),
    ];
    let mut lista2 = Vec::new();

    for palabra in lista1 {
        lista2.push(palabra.to_ascii_uppercase());
    }

    println!("Resultado: {:?}", lista2);
}
