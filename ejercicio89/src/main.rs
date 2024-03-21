use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio89.txt").expect("Error durante la creación");

    for x in 1..11 {
        let texto: String = format!("{}\n", "*".repeat(x));
        fichero
            .write_all(texto.as_bytes())
            .expect("Error durante la escritura");
    }
}
