/*Para una biblioteca se desea implementar un sistema de préstamos de libros. De la 
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para 
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica 
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De 
cada libro se conoce el isbn, el título, autor, número de páginas, género (novela, infantil, 
técnico, otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de 
vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en 
préstamo. Del cliente se conoce el nombre, teléfono y dirección de correo electrónico. 
Implemente los métodos necesarios para realizar las siguientes acciones:
➔  Obtener cantidad de copias: dado un determinado libro retorna la cantidad de 
copias a disposición que hay para prestar de dicho libro. 
➔  Decrementar cantidad de copias a disposición; dado un libro decrementa en 
1 la cantidad de copias de libros a disposición para prestar. 
➔  Incrementar cantidad de copias a disposición: dado un libro incrementa en 1 
la cantidad de copias del libro a disposición para ser prestado. 
➔  Contar préstamos de un cliente: devuelve la cantidad de préstamos en 
estado “en préstamo” de un determinado cliente. 
➔  Realizar un préstamo de un libro para un cliente: crea un préstamo de un 
libro para un determinado cliente cumpliendo con lo siguiente: 
◆  Que el cliente no tenga más de 5 préstamos en el estado “en 
préstamo”. 
◆  Que haya al menos una copia disponible en el registro de copias a 
disposición. De ser así descuenta 1 en el registro de “copias a 
disposición” y retorna true, si no cumple con alguna de las 
condiciones retorna false. 
➔  Ver préstamos a vencer el los próximos días: retorna una lista de préstamos 
a vencer el los próximos días, el valor de días es pasado por parámetro. 
➔  Ver los préstamos vencidos: retorna una lista de préstamos en el estado “en 
préstamo” donde la fecha de vencimiento es menor a la fecha actual. 
➔  Buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si 
existe. 
➔  Devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al 
estado “devuelto”, se registra la fecha de devolución y se incrementa la 
cantidad de libros en 1 del libro devuelto en el registro de copias a 
disposición. */

use crate::ejercicio3::Fecha;

pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub copias_disponibles: Vec<(Libro, u32)>,
    pub prestamos: Vec<Prestamo>,
}

#[derive(Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Clone)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
    pub paginas: u32,
    pub genero: Genero,
}

#[derive(Clone)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: String,
    pub email: String,
}

pub enum EstadoPrestamo {
    EnPrestamo,
    Devuelto,
}

pub struct Prestamo {
    pub libro: Libro,
    pub cliente: Cliente,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>,
    pub estado: EstadoPrestamo,
}

impl Biblioteca {
    pub fn obtener_copias(&self, libro: &Libro) -> Option<u32> {
        self.copias_disponibles.iter().find(|(l, _)| l.isbn == libro.isbn).map(|(_, copias)| *copias)
    }

    pub fn decrementar_copias(&mut self, libro: &Libro) -> bool {
        if let Some((_, copias)) = self.copias_disponibles.iter_mut().find(|(l, _)| l.isbn == libro.isbn) {
            if *copias > 0 {
                *copias -= 1;
                return true;
            }
        }
        false
    }

    pub fn incrementar_copias(&mut self, libro: &Libro) {
        if let Some((_, copias)) = self.copias_disponibles.iter_mut().find(|(l, _)| l.isbn == libro.isbn) {
            *copias += 1;
        }
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> usize {
        self.prestamos.iter().filter(|p| p.cliente.nombre == cliente.nombre && matches!(p.estado, EstadoPrestamo::EnPrestamo)).count()
    }

    pub fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> bool {
        if self.contar_prestamos_cliente(&cliente) >= 5 {
            return false;
        }
        if !self.decrementar_copias(&libro) {
            return false;
        }
        self.prestamos.push(Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            estado: EstadoPrestamo::EnPrestamo,
        });
        true
    }

    pub fn ver_prestamos_a_vencer(&self, fecha_actual: &Fecha, dias: u32) -> Vec<&Prestamo> {
        let mut fecha_limite = fecha_actual.clone();
        fecha_limite.sumar_dias(dias);
        self.prestamos
            .iter()
            .filter(|p|
                matches!(p.estado, EstadoPrestamo::EnPrestamo)
                && p.fecha_vencimiento.es_mayor(fecha_actual)
                && !p.fecha_vencimiento.es_mayor(&fecha_limite)
            )
            .collect()
    }

    pub fn ver_prestamos_vencidos(&self, fecha_actual: &Fecha) -> Vec<&Prestamo> {
        self.prestamos
            .iter()
            .filter(|p| matches!(p.estado, EstadoPrestamo::EnPrestamo) && !p.fecha_vencimiento.es_mayor(fecha_actual))
            .collect()
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        self.prestamos.iter().find(|p| p.libro.isbn == libro.isbn && p.cliente.nombre == cliente.nombre)
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_devolucion: Fecha) -> bool {
        if let Some(prestamo) = self.prestamos.iter_mut().find(|p| 
            p.libro.isbn == libro.isbn 
            && p.cliente.nombre == cliente.nombre 
            && matches!(p.estado, EstadoPrestamo::EnPrestamo)
        ) {
            prestamo.estado = EstadoPrestamo::Devuelto;
            prestamo.fecha_devolucion = Some(fecha_devolucion);
        }
        else {
            return false;
        }
        self.incrementar_copias(libro);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obtener_copias() {
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![],
        };
        assert_eq!(biblioteca.obtener_copias(&libro), Some(5));
    }

    #[test]
    fn test_decrementar_copias() {
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![],
        };
        assert!(biblioteca.decrementar_copias(&libro));
        assert_eq!(biblioteca.obtener_copias(&libro), Some(4));
    }

    #[test]
    fn test_incrementar_copias() {
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![],
        };
        biblioteca.incrementar_copias(&libro);
        assert_eq!(biblioteca.obtener_copias(&libro), Some(6));
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha::new(1, 1, 2023),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            }],
        };
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 1);
    }

    #[test]
    fn test_realizar_prestamo() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let fecha_vencimiento = Fecha::new(1, 1, 2023);
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![],
        };
        assert!(biblioteca.realizar_prestamo(libro, cliente, fecha_vencimiento));
    }

    #[test]
    fn test_realizar_prestamo_falla_si_no_hay_copias() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let fecha_vencimiento = Fecha::new(1, 1, 2023);
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 0)],
            prestamos: vec![],
        };

        assert!(!biblioteca.realizar_prestamo(libro, cliente, fecha_vencimiento));
        assert_eq!(biblioteca.prestamos.len(), 0);
    }

    #[test]
    fn test_realizar_prestamo_falla_si_cliente_tiene_cinco_prestamos() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let fecha_vencimiento = Fecha::new(1, 1, 2023);
        let mut prestamos = Vec::new();

        for _ in 0..5 {
            prestamos.push(Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha::new(1, 1, 2023),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            });
        }

        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos,
        };

        assert!(!biblioteca.realizar_prestamo(libro, cliente, fecha_vencimiento));
        assert_eq!(biblioteca.prestamos.len(), 5);
        assert_eq!(biblioteca.obtener_copias(&Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        }), Some(5));
    }

    #[test]
    fn test_ver_prestamos_a_vencer() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let fecha_vencimiento = Fecha::new(20, 1, 2023);
        let fecha_actual = Fecha::new(18, 1, 2023);
        let biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: fecha_vencimiento.clone(),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            }],
        };
        let prestamos_a_vencer = biblioteca.ver_prestamos_a_vencer(&fecha_actual, 5);
        assert_eq!(prestamos_a_vencer.len(), 1);
        assert_eq!(prestamos_a_vencer[0].libro.isbn, libro.isbn);
    }

    #[test]
    fn test_ver_prestamos_vencidos() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let fecha_vencimiento = Fecha::new(1, 1, 2023);
        let fecha_actual = Fecha::new(2, 1, 2023);
        let biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: fecha_vencimiento.clone(),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            }],
        };
        let prestamos_vencidos = biblioteca.ver_prestamos_vencidos(&fecha_actual);
        assert_eq!(prestamos_vencidos.len(), 1);
        assert_eq!(prestamos_vencidos[0].libro.isbn, libro.isbn);
    }

    #[test]
    fn test_buscar_prestamo() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha::new(1, 1, 2023),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            }],
        };
        let prestamo = biblioteca.buscar_prestamo(&libro, &cliente);
        assert!(prestamo.is_some());
        assert_eq!(prestamo.unwrap().libro.isbn, libro.isbn);
    }

    #[test]
    fn test_devolver_libro() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 4)],
            prestamos: vec![Prestamo {
                libro: libro.clone(),
                cliente: cliente.clone(),
                fecha_vencimiento: Fecha::new(1, 1, 2023),
                fecha_devolucion: None,
                estado: EstadoPrestamo::EnPrestamo,
            }],
        };
        assert!(biblioteca.devolver_libro(&libro, &cliente, Fecha::new(2, 1, 2023)));
        let prestamo = biblioteca.buscar_prestamo(&libro, &cliente).unwrap();
        assert!(matches!(prestamo.estado, EstadoPrestamo::Devuelto));
        if let Some(fecha_devolucion) = &prestamo.fecha_devolucion {
            let fecha_esperada = Fecha::new(2, 1, 2023);
            assert!(!fecha_devolucion.es_mayor(&fecha_esperada));
            assert!(!fecha_esperada.es_mayor(fecha_devolucion));
        } else {
            panic!("Se esperaba fecha de devolucion");
        }
        assert_eq!(biblioteca.obtener_copias(&libro), Some(5));
    }

    #[test]
    fn test_devolver_libro_falla_si_no_existe_prestamo() {
        let cliente = Cliente {
            nombre: "Cliente de prueba".to_string(),
            telefono: "123456789".to_string(),
            email: "cliente@prueba.com".to_string(),
        };
        let libro = Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor de prueba".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        };
        let mut biblioteca = Biblioteca {
            nombre: "Biblioteca de prueba".to_string(),
            direccion: "Dirección de prueba".to_string(),
            copias_disponibles: vec![(libro.clone(), 5)],
            prestamos: vec![],
        };

        assert!(!biblioteca.devolver_libro(&libro, &cliente, Fecha::new(2, 1, 2023)));
        assert_eq!(biblioteca.obtener_copias(&libro), Some(5));
    }
}
