use chrono;

fn main() {
    let fecha = chrono::offset::Local::now();
    println!("Fecha: {}", fecha);
}
