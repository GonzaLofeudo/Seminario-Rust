fn main() {
    const FACTOR: i32 = 4;

    let mut arreglo = [4, 8, 15, -6, 3, -1];

    println!("Arreglo original: {:?}", arreglo);

    for i in 0..arreglo.len() {
        arreglo[i] = arreglo[i] * FACTOR;
    }

    println!("Arreglo modificado: {:?}", arreglo);
}
