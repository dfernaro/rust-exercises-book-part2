fn main() {
    let lista: [i32; 4] = [12, 5, 3, 45];
    let mut maximo: i32 = lista[0];

    for num in lista {
        if num > maximo {
            maximo = num;
        }
    }

    println!("Máximo: {}", maximo);
}
