fn main() {
    let mut lista: Vec<i32> = Vec::new();

    for numero in 1..101 {
        if numero % 2 == 0 {
            lista.push(numero);
        }
    }

    println!("Resultado: {:?}", lista);
}
