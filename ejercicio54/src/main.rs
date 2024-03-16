fn main() {
    let lista: [i32; 7] = [4, 131, 1, 45, 131, 45, 4];
    let mut lista_sin_duplicados: Vec<i32> = Vec::new();

    for num in lista {
        if !lista_sin_duplicados.contains(&num) {
            lista_sin_duplicados.push(num);
        }
    }

    println!("Resultado: {:?}", lista_sin_duplicados);
}
