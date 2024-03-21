use std::io::Write;

fn main() {
    let mut fichero = std::fs::File::create("ejercicio96.txt").expect("Error durante la creación");

    let lista: [i32; 4] = [12, 5, 3, 45];
    let mut total: i32 = 0;

    for num in lista {
        total = total + num;
    }

    let resultado: String = format!("Suma total: {}", total);

    fichero
        .write_all(resultado.as_bytes())
        .expect("Error durante la escritura");
}
