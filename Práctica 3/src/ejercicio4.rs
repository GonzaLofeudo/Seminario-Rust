/*Escribir un programa que defina la estructura Triángulo que tenga campos para las 
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos: 
➢  new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna. 
➢  determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero, 
isósceles o escaleno. 
➢  calcular_area: calcular el área y la retorna. 
➢  calcular_perimetro: calcula el perímetro y lo retorna. */

#[derive(Debug, Clone)]
pub struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
}

impl Triangulo {
    pub fn new(lado1: f64, lado2: f64, lado3: f64) -> Self {
        Self {
            lado1,
            lado2,
            lado3,
        }
    }

    pub fn determinar_tipo(&self) -> &str {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            "equilátero"
        } else if self.lado1 == self.lado2 || self.lado2 == self.lado3 || self.lado1 == self.lado3 {
            "isósceles"
        } else {
            "escaleno"
        }
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }

    pub fn calcular_area(&self) -> f64 {
        let s = self.calcular_perimetro() / 2.0;
        (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let triangulo = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(triangulo.lado1, 3.0);
        assert_eq!(triangulo.lado2, 4.0);
        assert_eq!(triangulo.lado3, 5.0);
    }

    #[test]
    fn test_determinar_tipo() {
        let equilatero = Triangulo::new(3.0, 3.0, 3.0);
        assert_eq!(equilatero.determinar_tipo(), "equilátero");

        let isosceles = Triangulo::new(3.0, 3.0, 5.0);
        assert_eq!(isosceles.determinar_tipo(), "isósceles");

        let escaleno = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(escaleno.determinar_tipo(), "escaleno");
    }

    #[test]
    fn test_calcular_perimetro() {
        let triangulo = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(triangulo.calcular_perimetro(), 12.0);
    }

    #[test]
    fn test_calcular_area() {
        let triangulo = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(triangulo.calcular_area(), 6.0);
    }
}
