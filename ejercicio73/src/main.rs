fn factorial(factorial: i32) {
    let mut resultado: i32 = 1;

    if factorial < 0 {
        println!("El factorial no existe para números negativos");
    } else if factorial == 0 {
        println!("El factorial de 0 es 1");
    } else {
        for x in 1..factorial + 1 {
            resultado = resultado * x;
        }
        println!("El factorial de {} es {}", factorial, resultado);
    }
}

fn main() {
    factorial(5);
}
