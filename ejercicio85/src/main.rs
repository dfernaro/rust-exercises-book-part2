struct Cuadrado {
    lado: i32,
}

fn main() {
    let cuadrado = Cuadrado { lado: 15 };

    println!("Lado: {}", cuadrado.lado);
    println!("Area: {}", cuadrado.lado * cuadrado.lado);
}
