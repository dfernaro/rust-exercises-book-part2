fn main() {
    let lista1: [i32; 5] = [1, 2, 3, 4, 5];
    let mut lista2 = lista1;

    lista2.reverse();

    println!("Resultado: {:?}", lista2);
}
