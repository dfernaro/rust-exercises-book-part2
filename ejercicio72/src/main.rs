fn numeros_impares(limite: i32) -> Vec<i32> {
    let mut numeros: Vec<i32> = Vec::new();
    for numero in 1..limite + 1 {
        if numero % 2 == 1 {
            numeros.push(numero);
        }
    }
    numeros
}

fn main() {
    println!("{:?}", numeros_impares(20));
}
