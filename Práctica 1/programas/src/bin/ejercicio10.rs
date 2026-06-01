fn main() {
    let arreglo1: [i32; 5] = [1, 2, 3, 4, 5];
    let arreglo2: [i32; 5] = [6, 7, 8, 9, 10];
    let mut resultado: [i32; 5] = [0; 5];

    for i in 0..arreglo1.len() {
        resultado[i] = arreglo1[i] + arreglo2[i];
    }

    println!("El arreglo resultante es: {:?}", resultado);
}
