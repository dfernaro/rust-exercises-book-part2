fn main() {
    let nota: u32 = 96;

    if nota > 95 {
        println!("Tu calificación es: A+");
    } else if nota > 90 && nota <= 95 {
        println!("Tu calificación es: A");
    } else if nota > 80 && nota <= 90 {
        println!("Tu calificación es: B");
    } else if nota > 60 && nota <= 80 {
        println!("Tu calificación es: C");
    } else {
        println!("Tu calificación es: D");
    }
}
