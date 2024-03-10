fn main() {
    let importe: f32 = 950.0;
    let mut descuento: f32 = 0.0;

    if importe > 1000.0 {
        descuento = (40.0 / 100.0) * importe;
    } else if importe > 100.0 && importe <= 1000.0 {
        descuento = (15.0 / 100.0) * importe;
    } else {
        descuento = (5.0 / 100.0) * importe;
    }

    let total: f32 = importe - descuento;

    println!("Importe: {} | Importe neto: {}", importe, total);
}
