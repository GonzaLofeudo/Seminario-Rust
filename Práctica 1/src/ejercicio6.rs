use std::io::stdin;

fn main() {
    let numero: u32 = 10;

    println!("El valor inicial es: {}", numero);
    println!("Ingrese un numero entero sin signo para sumarle:");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let numero_ingresado: u32 = input
        .trim()
        .parse()
        .expect("Por favor, ingrese un numero valido");

    let resultado = numero + numero_ingresado;
    let resultado_cuadrado = resultado.pow(2);

    println!("El resultado de la suma es: {}", resultado);
    println!("El resultado elevado al cuadrado es: {}", resultado_cuadrado);
}