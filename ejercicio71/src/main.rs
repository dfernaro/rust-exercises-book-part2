fn par_o_impar(numero: i32) -> String {
    let mut resultado = String::from("Impar");
    if numero % 2 == 0 {
        resultado = String::from("Par");
    }
    resultado
}

fn main() {
    println!("{} es {}", 10, par_o_impar(10));
    println!("{} es {}", 7, par_o_impar(7));
}
