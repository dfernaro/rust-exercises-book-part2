use std::fs;

fn main() {
    let contenido_fichero =
        fs::read_to_string("ejercicio99.txt").expect("Error leyendo el fichero");
    println!("{}", contenido_fichero);
}
