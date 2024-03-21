use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio90.txt").expect("Error durante la creación");

    for _x in 1..6 {
        let texto: String = format!("{}\n", "*".repeat(5));
        fichero
            .write_all(texto.as_bytes())
            .expect("Error durante la escritura");
    }
}
