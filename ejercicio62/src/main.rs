fn main() {
    let mut numeros: Vec<i32> = Vec::new();

    for num in 1..21 {
        let cuadrado: i32 = num as i32;
        numeros.push(cuadrado.pow(2));
    }

    println!("{:?}", numeros);
}
