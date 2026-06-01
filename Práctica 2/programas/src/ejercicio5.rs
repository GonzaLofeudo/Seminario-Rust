pub fn duplicar_valores<const N: usize>(arreglo: &[f64; N]) -> [f64; N] {
    let mut resultado = [0.0; N];
    let mut indice = 0;

    while indice < N {
        resultado[indice] = arreglo[indice] * 2.0;
        indice += 1;
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicar_valores_con_flotantes_positivos() {
        let arreglo = [1.5, 2.0, 3.25];

        assert_eq!(duplicar_valores(&arreglo), [3.0, 4.0, 6.5]);
    }

    #[test]
    fn test_duplicar_valores_con_flotantes_negativos() {
        let arreglo = [-1.5, -2.0, 0.0];

        assert_eq!(duplicar_valores(&arreglo), [-3.0, -4.0, 0.0]);
    }

    #[test]
    fn test_duplicar_valores_con_arreglo_vacio() {
        let arreglo: [f64; 0] = [];
        let esperado: [f64; 0] = [];

        assert_eq!(duplicar_valores(&arreglo), esperado);
    }
}
