pub fn sumar_arreglos<const N: usize>(arreglo1: &[f64; N], arreglo2: &[f64; N]) -> [f64; N] {
    let mut resultado = [0.0; N];
    let mut indice = 0;

    while indice < N {
        resultado[indice] = arreglo1[indice] + arreglo2[indice];
        indice += 1;
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar_arreglos_con_mismo_tamano() {
        let arreglo1 = [1.5, 2.0, 3.25];
        let arreglo2 = [4.5, 3.0, 1.75];

        assert_eq!(sumar_arreglos(&arreglo1, &arreglo2), [6.0, 5.0, 5.0]);
    }

    #[test]
    fn test_sumar_arreglos_con_arreglos_vacios() {
        let arreglo1: [f64; 0] = [];
        let arreglo2: [f64; 0] = [];
        let esperado: [f64; 0] = [];

        assert_eq!(sumar_arreglos(&arreglo1, &arreglo2), esperado);
    }
}
