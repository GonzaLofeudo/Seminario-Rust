pub fn longitud_de_cadenas<const N: usize>(arreglo: &[String; N]) -> [usize; N] {
    let mut resultado = [0; N];
    let mut indice = 0;

    while indice < N {
        resultado[indice] = arreglo[indice].len();
        indice += 1;
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longitud_de_cadenas_con_varias_cadenas() {
        let arreglo = [
            "hola".to_string(),
            "rust".to_string(),
            "seminario".to_string(),
        ];

        assert_eq!(longitud_de_cadenas(&arreglo), [4, 4, 9]);
    }

    #[test]
    fn test_longitud_de_cadenas_con_cadena_vacia() {
        let arreglo = ["".to_string(), "abc".to_string()];

        assert_eq!(longitud_de_cadenas(&arreglo), [0, 3]);
    }

    #[test]
    fn test_longitud_de_cadenas_con_arreglo_vacio() {
        let arreglo: [String; 0] = [];
        let esperado: [usize; 0] = [];

        assert_eq!(longitud_de_cadenas(&arreglo), esperado);
    }
}
