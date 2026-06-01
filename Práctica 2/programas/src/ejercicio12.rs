use crate::ejercicio1::es_par;

pub fn reemplazar_pares(arreglo: &mut [i32]) {
    let mut indice = 0;

    while indice < arreglo.len() {
        if es_par(arreglo[indice]) {
            arreglo[indice] = -1;
        }

        indice += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reemplazar_pares_modifica_los_pares() {
        let mut arreglo = [1, 2, 3, 4, 5, 6];

        reemplazar_pares(&mut arreglo);

        assert_eq!(arreglo, [1, -1, 3, -1, 5, -1]);
    }

    #[test]
    fn test_reemplazar_pares_sin_numeros_pares() {
        let mut arreglo = [1, 3, 5];

        reemplazar_pares(&mut arreglo);

        assert_eq!(arreglo, [1, 3, 5]);
    }

    #[test]
    fn test_reemplazar_pares_con_cero() {
        let mut arreglo = [0, 1, 2];

        reemplazar_pares(&mut arreglo);

        assert_eq!(arreglo, [-1, 1, -1]);
    }

    #[test]
    fn test_reemplazar_pares_con_arreglo_vacio() {
        let mut arreglo: [i32; 0] = [];

        reemplazar_pares(&mut arreglo);

        assert_eq!(arreglo, []);
    }
}
