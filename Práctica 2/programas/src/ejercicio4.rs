use crate::ejercicio1::es_par;

pub fn cantidad_impares(arreglo: &[i32]) -> usize {
    let mut cantidad = 0;
    let mut indice = 0;

    while indice < arreglo.len() {
        if !es_par(arreglo[indice]) {
            cantidad += 1;
        }

        indice += 1;
    }

    cantidad
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantidad_impares_con_arreglo_mixto() {
        let arreglo = [1, 2, 3, 4, 5, 6];

        assert_eq!(cantidad_impares(&arreglo), 3);
    }

    #[test]
    fn test_cantidad_impares_sin_numeros_impares() {
        let arreglo = [2, 4, 6, 8];

        assert_eq!(cantidad_impares(&arreglo), 0);
    }

    #[test]
    fn test_cantidad_impares_con_arreglo_vacio() {
        let arreglo: [i32; 0] = [];

        assert_eq!(cantidad_impares(&arreglo), 0);
    }
}
