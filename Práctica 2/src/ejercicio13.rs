pub fn ordenar_nombres(arreglo: &mut [String]) {
    let len = arreglo.len();
    let mut i = 0;

    while i < len {
        let mut j = 0;

        while j + 1 < len - i {
            if arreglo[j] > arreglo[j + 1] {
                arreglo.swap(j, j + 1);
            }

            j += 1;
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordenar_nombres_ordena_alfabeticamente() {
        let mut nombres = ["Carla".to_string(), "Ana".to_string(), "Bruno".to_string()];

        ordenar_nombres(&mut nombres);

        assert_eq!(
            nombres,
            ["Ana".to_string(), "Bruno".to_string(), "Carla".to_string()]
        );
    }

    #[test]
    fn test_ordenar_nombres_con_arreglo_ya_ordenado() {
        let mut nombres = ["Ana".to_string(), "Bruno".to_string(), "Carla".to_string()];

        ordenar_nombres(&mut nombres);

        assert_eq!(
            nombres,
            ["Ana".to_string(), "Bruno".to_string(), "Carla".to_string()]
        );
    }

    #[test]
    fn test_ordenar_nombres_con_arreglo_vacio() {
        let mut nombres: [String; 0] = [];
        let esperado: [String; 0] = [];

        ordenar_nombres(&mut nombres);

        assert_eq!(nombres, esperado);
    }
}
