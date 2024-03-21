#[derive(Debug)]
enum Especialidad {
    IT,
    RECURSOS_HUMANOS,
    MARKETING,
}

struct Empleado {
    nombre: String,
    edad: u32,
    especialidad: Especialidad,
}

fn main() {
    let mut persona = Empleado {
        nombre: String::from("Marina"),
        edad: 34,
        especialidad: Especialidad::RECURSOS_HUMANOS,
    };

    persona.edad = 29;
    persona.especialidad = Especialidad::MARKETING;

    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);
    println!("Especialidad: {:?}", persona.especialidad);
}
