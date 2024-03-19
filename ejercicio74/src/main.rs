fn fibonacci(limite: i32) {
    let mut numero1: i32 = 0;
    let mut numero2: i32 = 1;

    for _i in 1..limite + 1 {
        print!("{} ", numero1);
        let resultado = numero1 + numero2;
        numero1 = numero2;
        numero2 = resultado;
    }
}

fn main() {
    fibonacci(8);
}
