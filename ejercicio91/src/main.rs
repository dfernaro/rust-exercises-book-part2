use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio91.txt").expect("Error durante la creación");
    let numero: i32 = 8;

    for x in 1..11 {
        let texto: String = format!("{} x {} = {}\n", numero, x, numero * x);
        fichero
            .write_all(texto.as_bytes())
            .expect("Error durante la escritura");
    }
}
