fn main() {
    let mut numero1: i32 = 0;
    let mut numero2: i32 = 1;

    for _x in 1..16 {
        print!("{} ", numero1);
        let resultado = numero1 + numero2;
        numero1 = numero2;
        numero2 = resultado;
    }
}
