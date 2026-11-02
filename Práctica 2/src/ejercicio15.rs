pub fn serie_geometrica<const N: usize>() -> [i32; N] {
    let mut resultado = [0; N];
    let mut indice = 0;
    let mut valor = 1;

    while indice < N {
        resultado[indice] = valor;
        valor *= 2;
        indice += 1;
    }

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serie_geometrica_con_tamano_positivo() {
        assert_eq!(serie_geometrica::<5>(), [1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_serie_geometrica_con_tamano_uno() {
        assert_eq!(serie_geometrica::<1>(), [1]);
    }

    #[test]
    fn test_serie_geometrica_con_tamano_cero() {
        let esperado: [i32; 0] = [];

        assert_eq!(serie_geometrica::<0>(), esperado);
    }
}
