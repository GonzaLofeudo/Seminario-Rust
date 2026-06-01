fn main() {
    let tupla: (&str, [i32; 5]) = ("Suma del arreglo", [1, 2, 3, 4, 5]);
    let mut suma: i32 = 0;

    for i in 0..tupla.1.len() {
        suma += tupla.1[i];
    }

    println!("{}: {}", tupla.0, suma);
}
