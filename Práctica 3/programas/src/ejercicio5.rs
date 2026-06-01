/*Escribir un programa que defina una estructura Producto que tenga campos para el 
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los 
siguientes métodos:
➢  new: que pasando los parámetros correspondientes, crea un Producto y lo retorna. 
➢  calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre el precio bruto. 
➢  aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de descuento sobre el precio bruto. 
➢  calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el precio total a pagar aplicando impuesto y descuento.
Tenga en cuenta que los parámetros son opcionales. */

#[derive(Debug, Clone)]
pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f64,
    pub id: u32,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f64, id: u32) -> Self {
        Self {
            nombre,
            precio_bruto,
            id,
        }
    }

    pub fn calcular_impuestos(&self, porcentaje_de_impuestos: f64) -> f64 {
        self.precio_bruto * porcentaje_de_impuestos / 100.0
    }

    pub fn aplicar_descuento(&self, porcentaje_de_descuento: f64) -> f64 {
        self.precio_bruto * porcentaje_de_descuento / 100.0
    }

    pub fn calcular_precio_total(
        &self,
        porcentaje_de_impuestos: Option<f64>,
        porcentaje_descuento: Option<f64>,
    ) -> f64 {
        let impuestos = porcentaje_de_impuestos.map_or(0.0, |p| self.calcular_impuestos(p));
        let descuento = porcentaje_descuento.map_or(0.0, |p| self.aplicar_descuento(p));
        self.precio_bruto + impuestos - descuento
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producto_new() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.nombre, "Laptop");
        assert_eq!(producto.precio_bruto, 1000.0);
        assert_eq!(producto.id, 1);
    }

    #[test]
    fn test_calcular_impuestos() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.calcular_impuestos(21.0), 210.0);
    }

    #[test]
    fn test_aplicar_descuento() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.aplicar_descuento(10.0), 100.0);
    }

    #[test]
    fn test_calcular_precio_total_con_impuestos_y_descuento() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(
            producto.calcular_precio_total(Some(21.0), Some(10.0)),
            1110.0
        );
    }

    #[test]
    fn test_calcular_precio_total_sin_impuestos_ni_descuento() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.calcular_precio_total(None, None), 1000.0);
    }

    #[test]
    fn test_calcular_precio_total_solo_impuestos() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.calcular_precio_total(Some(21.0), None), 1210.0);
    }

    #[test]
    fn test_calcular_precio_total_solo_descuento() {
        let producto = Producto::new("Laptop".to_string(), 1000.0, 1);

        assert_eq!(producto.calcular_precio_total(None, Some(10.0)), 900.0);
    }
}
