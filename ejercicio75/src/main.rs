fn mostrar_mensaje(nombre: String, apellido: String, edad: i32) -> String {
    let mensaje: String = format!("{} {} tiene {} años", nombre, apellido, edad);
    mensaje
}

fn main() {
    println!(
        "{}",
        mostrar_mensaje(String::from("Francisco"), String::from("Silva"), 18)
    );
}
