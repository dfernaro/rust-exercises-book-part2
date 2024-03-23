#[derive(Debug)]
enum Dias {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}

fn main() {
    println!("Día elegido: {:?}", Dias::Jueves);
}
