use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duenio {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u8,
    pub tipo: TipoAnimal,
    pub duenio: Duenio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub fecha_proxima_visita: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Veterinaria {
    pub id: u32,
    pub nombre: String,
    pub direccion: String,
    pub cola_atencion: VecDeque<Mascota>,
    pub registros_atencion: VecDeque<Atencion>,
    #[serde(skip)]
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VeterinariaError {
    Archivo(String),
    MascotaNoEncontrada(String),
    AtencionNoEncontrada(String),
}

impl Veterinaria {
    pub fn nueva(id: u32, nombre: impl Into<String>, direccion: impl Into<String>, archivo: impl Into<PathBuf>) -> Result<Self, VeterinariaError> {
        let mut veterinaria = Self {
            id,
            nombre: nombre.into(),
            direccion: direccion.into(),
            cola_atencion: VecDeque::new(),
            registros_atencion: VecDeque::new(),
            archivo: archivo.into(),
        };
        veterinaria.cargar_desde_archivo()?;
        Ok(veterinaria)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), VeterinariaError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            VeterinariaError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.cola_atencion = VecDeque::new();
            self.registros_atencion = VecDeque::new();
            return Ok(());
        }

        let veterinaria_guardada: Veterinaria = serde_json::from_str(&contenido).map_err(|e| {
            VeterinariaError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;

        self.cola_atencion = veterinaria_guardada.cola_atencion;
        self.registros_atencion = veterinaria_guardada.registros_atencion;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), VeterinariaError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    VeterinariaError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(self).map_err(|e| {
            VeterinariaError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            VeterinariaError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) -> Result<(), VeterinariaError> {
        self.cola_atencion.push_back(mascota);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) -> Result<(), VeterinariaError> {
        self.cola_atencion.push_front(mascota);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn atender_proxima_mascota(&mut self) -> Result<Option<Mascota>, VeterinariaError> {
        let mascota = self.cola_atencion.pop_front();
        self.guardar_en_archivo()?;
        Ok(mascota)
    }

    pub fn eliminar_mascota(&mut self, nombre: &str) -> Result<(), VeterinariaError> {
        let original = self.cola_atencion.len();
        self.cola_atencion.retain(|m| m.nombre != nombre);
        if self.cola_atencion.len() == original {
            return Err(VeterinariaError::MascotaNoEncontrada(format!(
                "No se encontró la mascota: {nombre}"
            )));
        }
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) -> Result<(), VeterinariaError> {
        self.registros_atencion.push_back(atencion);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str) -> Option<&Atencion> {
        self.registros_atencion.iter().find(|a| {
            a.mascota.nombre == nombre_mascota
                && a.mascota.duenio.nombre == nombre_duenio
                && a.mascota.duenio.telefono == telefono
        })
    }

    pub fn modificar_diagnostico(&mut self, nombre_mascota: &str, nuevo_diagnostico: String) -> Result<(), VeterinariaError> {
        let encontrado = self.registros_atencion.iter_mut().find(|a| a.mascota.nombre == nombre_mascota);
        if let Some(atencion) = encontrado {
            atencion.diagnostico = nuevo_diagnostico;
            self.guardar_en_archivo()?;
            Ok(())
        } else {
            Err(VeterinariaError::AtencionNoEncontrada(format!(
                "No se encontró la atención de: {nombre_mascota}"
            )))
        }
    }

    pub fn modificar_fecha_proxima_visita(&mut self, nombre_mascota: &str, nueva_fecha: String) -> Result<(), VeterinariaError> {
        let encontrado = self.registros_atencion.iter_mut().find(|a| a.mascota.nombre == nombre_mascota);
        if let Some(atencion) = encontrado {
            atencion.fecha_proxima_visita = Some(nueva_fecha);
            self.guardar_en_archivo()?;
            Ok(())
        } else {
            Err(VeterinariaError::AtencionNoEncontrada(format!(
                "No se encontró la atención de: {nombre_mascota}"
            )))
        }
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str) -> Result<(), VeterinariaError> {
        let original = self.registros_atencion.len();
        self.registros_atencion.retain(|a| a.mascota.nombre != nombre_mascota);
        if self.registros_atencion.len() == original {
            return Err(VeterinariaError::AtencionNoEncontrada(format!(
                "No se encontró la atención de: {nombre_mascota}"
            )));
        }
        self.guardar_en_archivo()?;
        Ok(())
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

    fn crear_duenio() -> Duenio {
        Duenio {
            nombre: "Ana".to_string(),
            direccion: "Calle 1".to_string(),
            telefono: "123456".to_string(),
        }
    }

    fn crear_mascota(nombre: &str) -> Mascota {
        Mascota {
            nombre: nombre.to_string(),
            edad: 4,
            tipo: TipoAnimal::Perro,
            duenio: crear_duenio(),
        }
    }

    fn crear_atencion(nombre_mascota: &str) -> Atencion {
        Atencion {
            mascota: crear_mascota(nombre_mascota),
            diagnostico: "Control".to_string(),
            tratamiento: "Reposo".to_string(),
            fecha_proxima_visita: None,
        }
    }

    #[test]
    fn test_nueva_veterinaria_y_carga() {
        let path = archivo_temporal("veterinaria_test");
        let _ = fs::remove_file(&path);

        let veterinaria = Veterinaria::nueva(1, "Vet Central", "Av. Siempreviva", &path).unwrap();

        assert_eq!(veterinaria.id, 1);
        assert!(veterinaria.cola_atencion.is_empty());
        assert!(veterinaria.registros_atencion.is_empty());
    }

    #[test]
    fn test_cola_y_atencion() {
        let path = archivo_temporal("veterinaria_cola");
        let _ = fs::remove_file(&path);

        let mut veterinaria = Veterinaria::nueva(1, "Vet Central", "Av. Siempreviva", &path).unwrap();
        veterinaria.agregar_mascota(crear_mascota("Luna")).unwrap();
        veterinaria.agregar_mascota_prioritaria(crear_mascota("Toby")).unwrap();

        let atendida = veterinaria.atender_proxima_mascota().unwrap().unwrap();
        assert_eq!(atendida.nombre, "Toby");
    }

    #[test]
    fn test_registrar_y_modificar_atencion() {
        let path = archivo_temporal("veterinaria_atencion");
        let _ = fs::remove_file(&path);

        let mut veterinaria = Veterinaria::nueva(1, "Vet Central", "Av. Siempreviva", &path).unwrap();
        veterinaria.registrar_atencion(crear_atencion("Luna")).unwrap();

        let encontrada = veterinaria.buscar_atencion("Luna", "Ana", "123456").unwrap();
        assert_eq!(encontrada.diagnostico, "Control");

        veterinaria
            .modificar_diagnostico("Luna", "Alta".to_string())
            .unwrap();
        veterinaria
            .modificar_fecha_proxima_visita("Luna", "2026-07-10".to_string())
            .unwrap();

        let actual = veterinaria.buscar_atencion("Luna", "Ana", "123456").unwrap();
        assert_eq!(actual.diagnostico, "Alta");
        assert_eq!(actual.fecha_proxima_visita.as_deref(), Some("2026-07-10"));

        veterinaria.eliminar_atencion("Luna").unwrap();
        assert!(veterinaria.registros_atencion.is_empty());
    }
}
