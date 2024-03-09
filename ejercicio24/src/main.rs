fn main() {
    let letra: char = 'E';

    if letra.to_ascii_lowercase() == 'a'
        || letra.to_ascii_lowercase() == 'e'
        || letra.to_ascii_lowercase() == 'i'
        || letra.to_ascii_lowercase() == 'o'
        || letra.to_ascii_lowercase() == 'u'
    {
        println!("La letra {} es una vocal", letra);
    } else {
        println!("La letra {} no es una vocal", letra);
    }
}
