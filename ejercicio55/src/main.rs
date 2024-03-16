fn main() {
    let lista: [i32; 3] = [25, 60, 30];
    let mut total: i32 = 0;

    for num in lista {
        total = total + num
    }

    let promedio = total as f32 / lista.len() as f32;

    println!("Promedio: {}", promedio);
}
