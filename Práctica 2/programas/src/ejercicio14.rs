pub fn incrementar(num: &mut f64) {
    *num += 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incrementar_numero_positivo() {
        let mut numero = 4.5;

        incrementar(&mut numero);

        assert_eq!(numero, 5.5);
    }

    #[test]
    fn test_incrementar_numero_negativo() {
        let mut numero = -2.0;

        incrementar(&mut numero);

        assert_eq!(numero, -1.0);
    }

    #[test]
    fn test_incrementar_cero() {
        let mut numero = 0.0;

        incrementar(&mut numero);

        assert_eq!(numero, 1.0);
    }
}
