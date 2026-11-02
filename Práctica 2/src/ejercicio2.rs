pub fn es_primo(num: i32) -> bool {
    if num <= 1 {
        panic!("El numero debe ser mayor a 1");
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_primo_con_numero_primo() {
        assert_eq!(es_primo(7), true);
    }

    #[test]
    fn test_es_primo_con_numero_compuesto() {
        assert_eq!(es_primo(9), false);
    }

    #[test]
    fn test_es_primo_con_dos() {
        assert_eq!(es_primo(2), true);
    }

    #[test]
    #[should_panic(expected = "El numero debe ser mayor a 1")]
    fn test_es_primo_con_uno() {
        es_primo(1);
    }
}
