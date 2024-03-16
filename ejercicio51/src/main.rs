fn main() {
    let lista: [i32; 4] = [12, 5, 3, 45];
    let mut total: i32 = 0;

    for num in lista {
        total = total + num;
    }

    println!("Suma total: {}", total);
}
