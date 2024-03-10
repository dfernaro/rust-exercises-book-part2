fn main() {
    let mut x: i32 = 6;
    let mut y: i32 = -9;

    if x > 0 || y > 0 {
        x = 10;
        y = y * 2;
    } else {
        y = y / 2;
        x = x * 2;
    }

    if x >= 10 {
        x = x.pow(2);
    }

    println!("{}", x);
    println!("{}", y);
}
