fn main() {
    let numero: i32 = 10;
    let mut total: i32 = 0;

    for x in 1..numero + 1 {
        total = total + x;
    }

    println!(
        "La suma de todos los números desde el 1 al {} es {}",
        numero, total
    );
}
