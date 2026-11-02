/* Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢  new: que pasando los parámetros correspondientes, crea un Rectángulo y lo retorna.
➢  calcular_area: calcular el área y la retorna.
➢  calcular_perimetro: calcula el perímetro y lo retorna.
➢  es_cuadrado: retorna true si es cuadrado, false caso contrario */

#[derive(Debug, Clone)]
pub struct Rectangulo {
    longitud: f64,
    ancho: f64,
}

impl Rectangulo {
    pub fn new(longitud: f64, ancho: f64) -> Self {
        Self { longitud, ancho }
    }

    pub fn calcular_area(&self) -> f64 {
        self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> f64 {
        2.0 * (self.longitud + self.ancho)
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangulo_new() {
        let rectangulo = Rectangulo::new(4.0, 2.0);

        assert_eq!(rectangulo.longitud, 4.0);
        assert_eq!(rectangulo.ancho, 2.0);
    }

    #[test]
    fn test_rectangulo_calcular_area() {
        let rectangulo = Rectangulo::new(4.0, 2.5);

        assert_eq!(rectangulo.calcular_area(), 10.0);
    }

    #[test]
    fn test_rectangulo_calcular_perimetro() {
        let rectangulo = Rectangulo::new(4.0, 2.0);

        assert_eq!(rectangulo.calcular_perimetro(), 12.0);
    }

    #[test]
    fn test_rectangulo_es_cuadrado() {
        let rectangulo = Rectangulo::new(3.0, 3.0);

        assert_eq!(rectangulo.es_cuadrado(), true);
    }

    #[test]
    fn test_rectangulo_no_es_cuadrado() {
        let rectangulo = Rectangulo::new(4.0, 2.0);

        assert_eq!(rectangulo.es_cuadrado(), false);
    }
}
