pub fn cantidad_de_cadenas_mayor_a(arreglo: &[String], limite: usize) -> usize {
    let mut cantidad = 0;
    let mut indice = 0;

    while indice < arreglo.len() {
        if arreglo[indice].len() > limite {
            cantidad += 1;
        }

        indice += 1;
    }

    cantidad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_con_cadenas_mixtas() {
        let arreglo = [
            "sol".to_string(),
            "luna".to_string(),
            "seminario".to_string(),
            "rust".to_string(),
        ];

        assert_eq!(cantidad_de_cadenas_mayor_a(&arreglo, 4), 1);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_no_cuenta_iguales_al_limite() {
        let arreglo = ["casa".to_string(), "perro".to_string(), "sol".to_string()];

        assert_eq!(cantidad_de_cadenas_mayor_a(&arreglo, 4), 1);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_sin_cadenas_mayores() {
        let arreglo = ["a".to_string(), "bb".to_string(), "ccc".to_string()];

        assert_eq!(cantidad_de_cadenas_mayor_a(&arreglo, 5), 0);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a_con_arreglo_vacio() {
        let arreglo: [String; 0] = [];

        assert_eq!(cantidad_de_cadenas_mayor_a(&arreglo, 3), 0);
    }
}
