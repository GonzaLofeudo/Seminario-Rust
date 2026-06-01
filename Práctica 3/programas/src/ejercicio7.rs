/*Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la 
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se 
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser: rojo, 
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos: 
❖  ConcesionarioAuto: 
➢  new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna. 
➢  agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar 
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere no lo agrega y retorna false. 
➢  eliminar_auto(auto): elimina un auto de la lista de autos. 
➢  buscar_auto(auto): busca un auto y si lo encuentra lo retorna. 
❖  Auto: 
➢  new: que pasando los parámetros correspondientes, crea un Auto y lo retorna. 
➢  calcular_precio: retorna el precio del auto aplicando los siguientes criterios: 
■  Si es de color primario le aplica un recargo del 25%, sino le aplica un 
descuento del 10%. 
■  Si la marca es BMW le aplica un recargo del 15%. 
■  Si el año es menor a 2000 se le aplica un descuento del 5%. */

pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_maxima: usize,
    pub autos: Vec<Auto>,
}

#[derive(Debug, Clone)]
pub struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Color,
}

#[derive(Debug, Clone)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

impl Color {
    fn es_primario(&self) -> bool {
        matches!(self, Color::Rojo | Color::Verde | Color::Azul)
    }
}

impl Auto {
    fn mismo_auto(&self, otro: &Auto) -> bool {
        self.marca == otro.marca
            && self.modelo == otro.modelo
            && self.anio == otro.anio
            && self.precio_bruto == otro.precio_bruto
            && matches!(
                (&self.color, &otro.color),
                (Color::Rojo, Color::Rojo)
                    | (Color::Verde, Color::Verde)
                    | (Color::Azul, Color::Azul)
                    | (Color::Amarillo, Color::Amarillo)
                    | (Color::Blanco, Color::Blanco)
                    | (Color::Negro, Color::Negro)
            )
    }
}

impl Auto {
    pub fn new(marca: String, modelo: String, anio: u32, precio_bruto: f64, color: Color) -> Self {
        Auto { marca, modelo, anio, precio_bruto, color }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut precio = self.precio_bruto;

        if self.color.es_primario() {
            precio *= 1.25; // Recargo del 25%
        } else {
            precio *= 0.9; // Descuento del 10%
        }

        if self.marca == "BMW" {
            precio *= 1.15; // Recargo del 15%
        }

        if self.anio < 2000 {
            precio *= 0.95; // Descuento del 5%
        }

        precio
    }
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: usize) -> Self {
        ConcesionarioAuto { nombre, direccion, capacidad_maxima, autos: Vec::new() }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad_maxima {
            self.autos.push(auto);
            true
        } else {
            false
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        if let Some(index) = self.autos.iter().position(|a| a.mismo_auto(auto)) {
            self.autos.remove(index);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|a| a.mismo_auto(auto))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_auto_de_prueba() -> Auto {
        Auto::new(
            "BMW".to_string(),
            "Serie 3".to_string(),
            1998,
            1000.0,
            Color::Rojo,
        )
    }

    #[test]
    fn test_auto_new() {
        let auto = crear_auto_de_prueba();

        assert_eq!(auto.marca, "BMW");
        assert_eq!(auto.modelo, "Serie 3");
        assert_eq!(auto.anio, 1998);
        assert_eq!(auto.precio_bruto, 1000.0);
        assert!(matches!(auto.color, Color::Rojo));
    }

    #[test]
    fn test_calcular_precio() {
        let auto = crear_auto_de_prueba();
        let precio = auto.calcular_precio();

        assert!((precio - 1365.625).abs() < 0.000001);
    }

    #[test]
    fn test_concesionario_new() {
        let concesionario = ConcesionarioAuto::new(
            "AutoSur".to_string(),
            "Calle 123".to_string(),
            2,
        );

        assert_eq!(concesionario.nombre, "AutoSur");
        assert_eq!(concesionario.direccion, "Calle 123");
        assert_eq!(concesionario.capacidad_maxima, 2);
        assert!(concesionario.autos.is_empty());
    }

    #[test]
    fn test_agregar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            "AutoSur".to_string(),
            "Calle 123".to_string(),
            1,
        );
        let auto = crear_auto_de_prueba();

        assert!(concesionario.agregar_auto(auto.clone()));
        assert!(!concesionario.agregar_auto(auto));
        assert_eq!(concesionario.autos.len(), 1);
    }

    #[test]
    fn test_buscar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            "AutoSur".to_string(),
            "Calle 123".to_string(),
            2,
        );
        let auto = crear_auto_de_prueba();

        concesionario.agregar_auto(auto.clone());

        let encontrado = concesionario.buscar_auto(&auto);

        assert!(encontrado.is_some());

        if let Some(auto_encontrado) = encontrado {
            assert_eq!(auto_encontrado.marca, auto.marca);
            assert_eq!(auto_encontrado.modelo, auto.modelo);
            assert_eq!(auto_encontrado.anio, auto.anio);
            assert_eq!(auto_encontrado.precio_bruto, auto.precio_bruto);
            assert!(matches!(auto_encontrado.color, Color::Rojo));
        }
    }

    #[test]
    fn test_eliminar_auto() {
        let mut concesionario = ConcesionarioAuto::new(
            "AutoSur".to_string(),
            "Calle 123".to_string(),
            2,
        );
        let auto = crear_auto_de_prueba();

        concesionario.agregar_auto(auto.clone());

        assert!(concesionario.eliminar_auto(&auto));
        assert!(!concesionario.eliminar_auto(&auto));
        assert!(concesionario.autos.is_empty());
    }
}
