use std::io::stdin;

fn main() {
    let valor: bool = true;

    println!("El valor inicial es: {}", valor);
    println!("Ingrese un valor booleano (true/false):");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let valor_ingresado: bool = input
        .trim()
        .parse()
        .expect("Por favor, ingrese un valor valido");

    println!("Operacion AND: {}", valor && valor_ingresado);
    println!("Operacion OR: {}", valor || valor_ingresado);
}
