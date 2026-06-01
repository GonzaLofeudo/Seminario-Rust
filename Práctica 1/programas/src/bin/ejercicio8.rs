use std::io::stdin;

fn main() {
    const CADENA: &str = "Seminario de Lenguajes, opcion Rust";

    println!("Ingrese un caracter para buscar en la cadena '{}':", CADENA);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let caracter: char = input
        .trim()
        .chars()
        .next()
        .expect("Por favor, ingrese un caracter valido");

    let mut contador = 0;

    for c in CADENA.chars() {
        if c == caracter {
            contador += 1;
        }
    }

    println!(
        "El caracter '{}' aparece {} veces en la cadena '{}'",
        caracter, contador, CADENA
    );
}
