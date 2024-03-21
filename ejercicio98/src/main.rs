use std::io::Write;

fn main() {
    let fichero = std::fs::File::create("ejercicio98.txt").expect("Error durante la creación");

    fichero
        .write_all("Ejemplo!".as_bytes())
        .expect("Error durante la escritura");
}
