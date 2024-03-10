fn main() {
    let numero: i32 = 5;

    for x in 1..numero + 1 {
        println!("El cubo de {} es {}", x, x.pow(3))
    }
}
