use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
    pub paginas: u32,
    pub genero: Genero,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EstadoPrestamo {
    EnPrestamo,
    Devuelto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prestamo {
    pub libro: Libro,
    pub cliente: Cliente,
    pub fecha_vencimiento: String,
    pub fecha_devolucion: Option<String>,
    pub estado: EstadoPrestamo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub copias_disponibles: Vec<(Libro, u32)>,
    pub prestamos: Vec<Prestamo>,
    #[serde(skip)]
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BibliotecaError {
    Archivo(String),
    LibroNoEncontrado(String),
    PrestamoNoEncontrado(String),
    LimitePrestamos(String),
}

impl Biblioteca {
    pub fn nueva(nombre: impl Into<String>, direccion: impl Into<String>, archivo: impl Into<PathBuf>) -> Result<Self, BibliotecaError> {
        let mut biblioteca = Self {
            nombre: nombre.into(),
            direccion: direccion.into(),
            copias_disponibles: Vec::new(),
            prestamos: Vec::new(),
            archivo: archivo.into(),
        };
        biblioteca.cargar_desde_archivo()?;
        Ok(biblioteca)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), BibliotecaError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            BibliotecaError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.copias_disponibles = Vec::new();
            self.prestamos = Vec::new();
            return Ok(());
        }

        let biblioteca_guardada: Biblioteca = serde_json::from_str(&contenido).map_err(|e| {
            BibliotecaError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;

        self.nombre = biblioteca_guardada.nombre;
        self.direccion = biblioteca_guardada.direccion;
        self.copias_disponibles = biblioteca_guardada.copias_disponibles;
        self.prestamos = biblioteca_guardada.prestamos;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), BibliotecaError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    BibliotecaError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(self).map_err(|e| {
            BibliotecaError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            BibliotecaError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn agregar_copia(&mut self, libro: Libro, copias: u32) -> Result<(), BibliotecaError> {
        self.copias_disponibles.push((libro, copias));
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn obtener_copias(&self, libro: &Libro) -> Option<u32> {
        self.copias_disponibles
            .iter()
            .find(|(libro_actual, _)| libro_actual.isbn == libro.isbn)
            .map(|(_, copias)| *copias)
    }

    pub fn decrementar_copias(&mut self, libro: &Libro) -> Result<(), BibliotecaError> {
        if let Some((_, copias)) = self.copias_disponibles.iter_mut().find(|(libro_actual, _)| libro_actual.isbn == libro.isbn) {
            if *copias > 0 {
                *copias -= 1;
                self.guardar_en_archivo()?;
                return Ok(());
            }
        }
        Err(BibliotecaError::LibroNoEncontrado(format!(
            "No hay copias disponibles para: {}",
            libro.titulo
        )))
    }

    pub fn incrementar_copias(&mut self, libro: &Libro) -> Result<(), BibliotecaError> {
        if let Some((_, copias)) = self.copias_disponibles.iter_mut().find(|(libro_actual, _)| libro_actual.isbn == libro.isbn) {
            *copias += 1;
            self.guardar_en_archivo()?;
            return Ok(());
        }
        Err(BibliotecaError::LibroNoEncontrado(format!(
            "No se encontró el libro: {}",
            libro.titulo
        )))
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> usize {
        self.prestamos
            .iter()
            .filter(|p| p.cliente.nombre == cliente.nombre && p.estado == EstadoPrestamo::EnPrestamo)
            .count()
    }

    pub fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fecha_vencimiento: String) -> Result<bool, BibliotecaError> {
        if self.contar_prestamos_cliente(&cliente) >= 5 {
            return Ok(false);
        }

        let tiene_copia = self
            .copias_disponibles
            .iter_mut()
            .find(|(libro_actual, _)| libro_actual.isbn == libro.isbn)
            .map(|(_, copias)| {
                if *copias > 0 {
                    *copias -= 1;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);

        if !tiene_copia {
            return Ok(false);
        }

        self.prestamos.push(Prestamo {
            libro: libro.clone(),
            cliente: cliente.clone(),
            fecha_vencimiento: fecha_vencimiento.clone(),
            fecha_devolucion: None,
            estado: EstadoPrestamo::EnPrestamo,
        });
        self.guardar_en_archivo()?;
        Ok(true)
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        self.prestamos.iter().find(|p| {
            p.libro.isbn == libro.isbn && p.cliente.nombre == cliente.nombre
        })
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_devolucion: String) -> Result<bool, BibliotecaError> {
        if let Some(prestamo) = self.prestamos.iter_mut().find(|p| {
            p.libro.isbn == libro.isbn && p.cliente.nombre == cliente.nombre && p.estado == EstadoPrestamo::EnPrestamo
        }) {
            prestamo.estado = EstadoPrestamo::Devuelto;
            prestamo.fecha_devolucion = Some(fecha_devolucion);
            self.incrementar_copias(libro)?;
            self.guardar_en_archivo()?;
            Ok(true)
        } else {
            Err(BibliotecaError::PrestamoNoEncontrado(format!(
                "No se encontró un préstamo activo para: {}",
                libro.titulo
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn archivo_temporal(nombre: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("{}_{}.json", nombre, std::process::id()));
        path
    }

    fn libro_prueba() -> Libro {
        Libro {
            isbn: "123".to_string(),
            titulo: "Libro de prueba".to_string(),
            autor: "Autor".to_string(),
            paginas: 100,
            genero: Genero::Novela,
        }
    }

    fn cliente_prueba() -> Cliente {
        Cliente {
            nombre: "Ana".to_string(),
            telefono: "123".to_string(),
            email: "ana@test.com".to_string(),
        }
    }

    #[test]
    fn test_prestamo_y_devolucion() {
        let path = archivo_temporal("biblioteca_test");
        let _ = fs::remove_file(&path);

        let mut biblioteca = Biblioteca::nueva("Biblioteca", "Calle 1", &path).unwrap();
        let libro = libro_prueba();
        biblioteca.agregar_copia(libro.clone(), 2).unwrap();

        assert_eq!(biblioteca.obtener_copias(&libro), Some(2));
        assert!(biblioteca.realizar_prestamo(libro.clone(), cliente_prueba(), "2026-07-10".to_string()).unwrap());
        assert_eq!(biblioteca.obtener_copias(&libro), Some(1));

        let prestamo = biblioteca.buscar_prestamo(&libro, &cliente_prueba()).unwrap();
        assert_eq!(prestamo.estado, EstadoPrestamo::EnPrestamo);

        biblioteca.devolver_libro(&libro, &cliente_prueba(), "2026-07-11".to_string()).unwrap();
        assert_eq!(biblioteca.obtener_copias(&libro), Some(2));
    }

    #[test]
    fn test_limite_prestamos_y_sin_copias() {
        let path = archivo_temporal("biblioteca_limite");
        let _ = fs::remove_file(&path);

        let mut biblioteca = Biblioteca::nueva("Biblioteca", "Calle 1", &path).unwrap();
        let libro = libro_prueba();
        biblioteca.agregar_copia(libro.clone(), 6).unwrap();

        let cliente = cliente_prueba();
        for _ in 0..5 {
            let ok = biblioteca
                .realizar_prestamo(libro.clone(), cliente.clone(), "2026-07-10".to_string())
                .unwrap();
            assert!(ok);
        }

        let ok = biblioteca
            .realizar_prestamo(libro.clone(), cliente, "2026-07-10".to_string())
            .unwrap();
        assert!(!ok);
    }
}
