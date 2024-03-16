fn main() {
    let lista = ["Rojo", "Blanco", "Azul", "Rojo", "Negro"];
    let mut total: i32 = 0;

    for color in lista {
        if color == "Rojo" {
            total = total + 1;
        }
    }

    println!("El color Rojo aparece {} veces", total);
}
