pub fn cantidad_en_rango(arreglo: &[i32], inferior: i32, superior: i32) -> usize {
    if inferior > superior {
        panic!("El limite inferior no puede ser mayor al superior");
    }

    let mut cantidad = 0;
    let mut indice = 0;

    while indice < arreglo.len() {
        if arreglo[indice] >= inferior && arreglo[indice] <= superior {
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
    fn test_cantidad_en_rango_con_valores_en_rango() {
        let arreglo = [1, 3, 5, 7, 9];

        assert_eq!(cantidad_en_rango(&arreglo, 3, 7), 3);
    }

    #[test]
    fn test_cantidad_en_rango_incluye_extremos() {
        let arreglo = [2, 4, 6, 8];

        assert_eq!(cantidad_en_rango(&arreglo, 4, 8), 3);
    }

    #[test]
    fn test_cantidad_en_rango_sin_valores_en_rango() {
        let arreglo = [10, 20, 30];

        assert_eq!(cantidad_en_rango(&arreglo, 1, 5), 0);
    }

    #[test]
    fn test_cantidad_en_rango_con_arreglo_vacio() {
        let arreglo: [i32; 0] = [];

        assert_eq!(cantidad_en_rango(&arreglo, 1, 5), 0);
    }

    #[test]
    #[should_panic(expected = "El limite inferior no puede ser mayor al superior")]
    fn test_cantidad_en_rango_con_limites_invalidos() {
        let arreglo = [1, 2, 3];

        cantidad_en_rango(&arreglo, 10, 5);
    }
}
