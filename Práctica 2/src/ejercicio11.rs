pub fn multiplicar_valores(arreglo: &mut [i32], factor: i32) {
    let mut indice = 0;

    while indice < arreglo.len() {
        arreglo[indice] *= factor;
        indice += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplicar_valores_modifica_el_arreglo() {
        let mut arreglo = [1, 2, 3, 4];

        multiplicar_valores(&mut arreglo, 3);

        assert_eq!(arreglo, [3, 6, 9, 12]);
    }

    #[test]
    fn test_multiplicar_valores_con_factor_cero() {
        let mut arreglo = [1, -2, 3];

        multiplicar_valores(&mut arreglo, 0);

        assert_eq!(arreglo, [0, 0, 0]);
    }

    #[test]
    fn test_multiplicar_valores_con_arreglo_vacio() {
        let mut arreglo: [i32; 0] = [];

        multiplicar_valores(&mut arreglo, 5);

        assert_eq!(arreglo, []);
    }
}
