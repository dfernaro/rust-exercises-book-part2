fn comprobar_elemento(elementos: Vec<String>, elemento: String) -> bool {
    let mut resultado: bool = false;

    if elementos.contains(&elemento) {
        resultado = true;
    }

    resultado
}

fn main() {
    let comprobar: bool = comprobar_elemento(
        Vec::from([String::from("Francisco"), String::from("Margarita")]),
        String::from("Margarita"),
    );

    println!(
        "¿Está {} en la lista {:?}? - Respuesta: {}",
        String::from("Margarita"),
        Vec::from([String::from("Francisco"), String::from("Margarita")]),
        comprobar
    );
}
