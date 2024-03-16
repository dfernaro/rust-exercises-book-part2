fn main() {
    let vehiculos: (String, String, String) = (
        String::from("coche"),
        String::from("camión"),
        String::from("helicóptero"),
    );

    println!("Vehículo: {}", vehiculos.0);
    println!("Vehículo: {}", vehiculos.1);
    println!("Vehículo: {}", vehiculos.2);
}
