use std::io::stdin;

fn main() {
    let mensaje = "Hola".to_string();

    println!("Ingrese un mensaje para concatenar con '{}':", mensaje);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error al leer la entrada");

    let mut resultado = mensaje;
    resultado.push(' ');
    resultado.push_str(input.trim());

    println!("El resultado en mayusculas es: {}", resultado.to_uppercase());
}
