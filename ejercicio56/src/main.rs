fn main() {
    let mut lista: [i32; 5] = [1, 2, 3, 4, 5];

    for num in lista.iter_mut() {
        *num = num.pow(3);
    }

    println!("Resultado: {:?}", lista);
}
