use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentMethod {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cliente {
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub dni: String,
    pub newsletter: bool,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vendedor {
    pub legajo: u32,
    pub antiguedad: u32,
    pub salario: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Producto {
    pub nombre: String,
    pub categoria: String,
    pub precio_base: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Venta {
    pub fecha: String,
    pub cliente: Cliente,
    pub vendedor: Vendedor,
    pub medio_pago: PaymentMethod,
    pub productos: Vec<(Producto, u32)>,
}

#[derive(Debug, Clone, Default)]
pub struct SalesSystem {
    pub ventas: Vec<Venta>,
    pub descuentos_por_categoria: HashMap<String, f64>,
}

impl SalesSystem {
    pub fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }

    pub fn precio_final(&self, venta: &Venta) -> f64 {
        let subtotal = venta.productos.iter().fold(0.0, |acc, (producto, cantidad)| {
            let base = producto.precio_base * f64::from(*cantidad);
            let descuento = self
                .descuentos_por_categoria
                .get(&producto.categoria)
                .copied()
                .unwrap_or(0.0);
            acc + base * (1.0 - descuento / 100.0)
        });

        let descuento_newsletter = if venta.cliente.newsletter { 10.0 } else { 0.0 };
        subtotal * (1.0 - descuento_newsletter / 100.0)
    }

    pub fn reporte_por_categoria(&self) -> HashMap<String, f64> {
        let mut reporte = HashMap::new();
        for venta in &self.ventas {
            for (producto, cantidad) in &venta.productos {
                let base = producto.precio_base * f64::from(*cantidad);
                let descuento = self
                    .descuentos_por_categoria
                    .get(&producto.categoria)
                    .copied()
                    .unwrap_or(0.0);
                let total_producto = base * (1.0 - descuento / 100.0);
                let entry = reporte.entry(producto.categoria.clone()).or_insert(0.0);
                *entry += total_producto;
            }
        }
        reporte
    }

    pub fn reporte_por_vendedor(&self) -> HashMap<String, f64> {
        let mut reporte = HashMap::new();
        for venta in &self.ventas {
            let total = self.precio_final(venta);
            let entry = reporte.entry(venta.vendedor.legajo.to_string()).or_insert(0.0);
            *entry += total;
        }
        reporte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ventas() {
        let mut system = SalesSystem::default();
        system.descuentos_por_categoria.insert("Electrónica".to_string(), 10.0);

        let venta = Venta {
            fecha: "2026-07-01".to_string(),
            cliente: Cliente {
                nombre: "Ana".to_string(),
                apellido: "García".to_string(),
                direccion: "Calle 1".to_string(),
                dni: "123".to_string(),
                newsletter: false,
                email: Some("ana@test.com".to_string()),
            },
            vendedor: Vendedor {
                legajo: 10,
                antiguedad: 2,
                salario: 100000.0,
            },
            medio_pago: PaymentMethod::TarjetaCredito,
            productos: vec![
                (
                    Producto {
                        nombre: "Notebook".to_string(),
                        categoria: "Electrónica".to_string(),
                        precio_base: 1000.0,
                    },
                    1,
                ),
            ],
        };

        system.registrar_venta(venta.clone());
        assert_eq!(system.precio_final(&venta), 900.0);
        assert_eq!(system.reporte_por_categoria()["Electrónica"], 900.0);
        assert_eq!(system.reporte_por_vendedor()["10"], 900.0);
    }
}
