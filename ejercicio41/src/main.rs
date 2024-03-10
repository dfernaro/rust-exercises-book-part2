fn main() {
    for x in 1..10 {
        let num: String = x.to_string();
        println!("{}", num.repeat(x));
    }
}
