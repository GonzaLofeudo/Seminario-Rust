use crate::ejercicio1::es_par;

pub fn suma_pares(arreglo: &[i32]) -> i32 {
    let mut suma = 0;
    let mut indice = 0;

    while indice < arreglo.len() {
        if es_par(arreglo[indice]) {
            suma += arreglo[indice];
        }

        indice += 1;
    }

    suma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suma_pares_con_arreglo_mixto() {
        let arreglo = [1, 2, 3, 4, 5, 6];

        assert_eq!(suma_pares(&arreglo), 12);
    }

    #[test]
    fn test_suma_pares_sin_numeros_pares() {
        let arreglo = [1, 3, 5, 7];

        assert_eq!(suma_pares(&arreglo), 0);
    }

    #[test]
    fn test_suma_pares_con_arreglo_vacio() {
        let arreglo: [i32; 0] = [];

        assert_eq!(suma_pares(&arreglo), 0);
    }
}
