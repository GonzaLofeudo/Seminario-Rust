pub fn es_par(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_par_con_numero_par() {
        assert_eq!(es_par(4), true);
    }

    #[test]
    fn test_es_par_con_numero_impar() {
        assert_eq!(es_par(7), false);
    }

    #[test]
    fn test_es_par_con_cero() {
        assert_eq!(es_par(0), true);
    }
}
