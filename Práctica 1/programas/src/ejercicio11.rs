use std::io::stdin;

fn main() {
    let frutas: [&str; 5] = ["manzana", "banana", "naranja", "pera", "uva"];

    println!("Ingrese una fruta para comprobar si esta en el arreglo:");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let fruta: &str = input.trim();

    let mut encontrado = false;

    for fruta_arreglo in frutas {
        if fruta_arreglo == fruta {
            encontrado = true;
            break;
        }
    }
    if encontrado {
        println!("La fruta '{}' esta en el arreglo.", fruta);
    } else {
        println!("La fruta '{}' no esta en el arreglo.", fruta);
    }
}
