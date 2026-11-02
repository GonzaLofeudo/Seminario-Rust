use std::io::stdin;

fn main() {
    let numero: f32 = 5.0;

    println!("El valor inicial es: {}", numero);
    println!("Ingrese un numero decimal:");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let numero_ingresado: f32 = input
        .trim()
        .parse()
        .expect("Por favor, ingrese un numero valido");

    println!("Suma: {}", numero + numero_ingresado);
    println!("Resta: {}", numero - numero_ingresado);
    println!("Multiplicacion: {}", numero * numero_ingresado);
    println!("Division: {}", numero / numero_ingresado);
}