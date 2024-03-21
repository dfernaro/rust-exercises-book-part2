use std::io::Write;

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
    let mut fichero = std::fs::File::create("ejercicio94.txt").expect("Error durante la creación");
    let mut persona = Empleado {
        nombre: String::from("Marina"),
        edad: 34,
        especialidad: Especialidad::RECURSOS_HUMANOS,
    };

    persona.edad = 29;
    persona.especialidad = Especialidad::MARKETING;

    let nombre: String = format!("Nombre: {}\n", persona.nombre);
    let edad: String = format!("Edad: {}\n", persona.edad);
    let especialidad: String = format!("Especialidad: {:?}\n", persona.especialidad);

    fichero
        .write_all(nombre.as_bytes())
        .expect("Error durante la escritura");
    fichero
        .write_all(edad.as_bytes())
        .expect("Error durante la escritura");
    fichero
        .write_all(especialidad.as_bytes())
        .expect("Error durante la escritura");
}
