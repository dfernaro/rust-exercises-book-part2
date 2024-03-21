use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio97.txt").expect("Error durante la creación");

    let lista = ["Rojo", "Blanco", "Azul", "Rojo", "Negro"];
    let mut total: i32 = 0;

    for color in lista {
        if color == "Rojo" {
            total = total + 1;
        }
    }

    let resultado: String = format!("El color Rojo aparece {} veces", total);

    fichero
        .write_all(resultado.as_bytes())
        .expect("Error durante la escritura");
}
