fn main() {
    let numero1: i32 = 2;
    let numero2: i32 = 3;
    let operador: char = '+';

    if operador == '+' {
        println!("Resultado: {}", numero1 + numero2);
    } else if operador == '-' {
        println!("Resultado: {}", numero1 - numero2);
    } else if operador == '*' {
        println!("Resultado: {}", numero1 * numero2);
    } else if operador == '/' {
        println!("Resultado: {}", numero1 as f32 / numero2 as f32);
    } else {
        println!("Operador no válido");
    }
}
