fn main() {
    let x: i32 = 2;
    let y: i32 = 3;

    let suma: i32 = x + y;
    let resta: i32 = x - y;
    let multiplicacion: i32 = x * y;
    let division: f32 = x as f32 / y as f32;
    let modulo: i32 = x % y;
    let exponente: i32 = x.pow(y as u32);

    println!("{}", suma);
    println!("{}", resta);
    println!("{}", multiplicacion);
    println!("{}", division);
    println!("{}", modulo);
    println!("{}", exponente);
}
