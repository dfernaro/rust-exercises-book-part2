struct Circulo {
    radio: f32,
}

fn main() {
    let circulo = Circulo { radio: 2.0 };

    println!("Radio: {}", circulo.radio);
    println!("Area: {}", circulo.radio * circulo.radio * 3.14);
    println!("Perimetro: {}", 2.0 * circulo.radio * 3.14);
}
