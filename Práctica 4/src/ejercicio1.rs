pub trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if *self < 2 {
            return false;
        }
        if *self == 2 {
            return true;
        }
        if *self % 2 == 0 {
            return false;
        }

        let mut divisor = 3;
        while divisor * divisor <= *self {
            if *self % divisor == 0 {
                return false;
            }
            divisor += 2;
        }

        true
    }
}

pub fn contar_primos(numeros: &[i32]) -> usize {
    numeros.iter().filter(|numero| numero.es_primo()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contar_primos() {
        assert_eq!(contar_primos(&[2, 3, 4, 5, 7, 9, 11]), 5);
    }
}
