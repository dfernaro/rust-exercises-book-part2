fn average(numeros: Vec<i32>) -> f32 {
    let mut suma: i32 = 0;
    let total: i32 = numeros.len() as i32;

    for numero in numeros {
        suma = suma + numero;
    }

    let resultado: f32 = suma as f32 / total as f32;

    resultado
}

fn main() {
    println!(
        "La media de {:?} es {}",
        Vec::from([15, 65, 20]),
        average(Vec::from([15, 65, 20]))
    );
}
