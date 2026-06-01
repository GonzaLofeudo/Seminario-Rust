pub fn cantidad_de_mayores(arreglo: &[i32], limite: i32) -> usize {
    let mut cantidad = 0;
    let mut indice = 0;

    while indice < arreglo.len() {
        if arreglo[indice] > limite {
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
    fn test_cantidad_de_mayores_con_arreglo_mixto() {
        let arreglo = [3, 8, 10, 2, 15];

        assert_eq!(cantidad_de_mayores(&arreglo, 7), 3);
    }

    #[test]
    fn test_cantidad_de_mayores_no_cuenta_iguales_al_limite() {
        let arreglo = [5, 6, 7, 8];

        assert_eq!(cantidad_de_mayores(&arreglo, 6), 2);
    }

    #[test]
    fn test_cantidad_de_mayores_sin_mayores() {
        let arreglo = [1, 2, 3];

        assert_eq!(cantidad_de_mayores(&arreglo, 10), 0);
    }

    #[test]
    fn test_cantidad_de_mayores_con_arreglo_vacio() {
        let arreglo: [i32; 0] = [];

        assert_eq!(cantidad_de_mayores(&arreglo, 5), 0);
    }
}
