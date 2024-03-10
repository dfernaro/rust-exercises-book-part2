fn main() {
    for x in (0..10).rev() {
        for y in 1..x + 1 {
            print!("{} ", y);
        }
        println!("");
    }
}
