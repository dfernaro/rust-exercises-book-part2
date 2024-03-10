fn main() {
    let mut x: i32 = 86;
    let mut y: i32 = -99;

    if x % 2 == 0 {
        x = y;
        y = y * 2;
    } else {
        y = x / 2;
        x = x * 2;
    }

    println!("{}", x);
    println!("{}", y);
}
