use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio95.txt").expect("Error durante la creación");

    for x in 1..10 {
        let num: String = x.to_string();
        let texto: String = format!("{}\n", num.repeat(x));
        fichero
            .write_all(texto.as_bytes())
            .expect("Error durante la escritura");
    }
}
