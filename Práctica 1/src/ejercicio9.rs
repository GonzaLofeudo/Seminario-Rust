fn main() {
    let arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    let mut suma: i32 = 0;

    for i in 0..arreglo.len() {
        suma += arreglo[i];
    }

    println!("La suma de los elementos del arreglo es: {}", suma);
}
