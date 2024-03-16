fn mayor(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("Mayor valor pasado por parámetros: {}", mayor(10, 20));
}
