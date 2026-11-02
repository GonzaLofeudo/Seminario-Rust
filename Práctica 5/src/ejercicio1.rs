use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub anio: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Concesionaria {
    pub autos: Vec<Auto>,
    pub limite: usize,
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConcesionariaError {
    LimiteAlcanzado(String),
    IndiceInvalido(String),
    Archivo(String),
}

impl Concesionaria {
    pub fn nuevo(limite: usize, archivo: impl Into<PathBuf>) -> Result<Self, ConcesionariaError> {
        let archivo = archivo.into();
        let mut concesionaria = Self {
            autos: Vec::new(),
            limite,
            archivo: archivo.clone(),
        };
        concesionaria.cargar_desde_archivo()?;
        Ok(concesionaria)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), ConcesionariaError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            ConcesionariaError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.autos = Vec::new();
            return Ok(());
        }

        self.autos = serde_json::from_str(&contenido).map_err(|e| {
            ConcesionariaError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), ConcesionariaError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    ConcesionariaError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(&self.autos).map_err(|e| {
            ConcesionariaError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            ConcesionariaError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> Result<(), ConcesionariaError> {
        if self.autos.len() >= self.limite {
            return Err(ConcesionariaError::LimiteAlcanzado(format!(
                "Se alcanzó el límite de {} autos",
                self.limite
            )));
        }

        self.autos.push(auto);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn eliminar_auto(&mut self, indice: usize) -> Result<(), ConcesionariaError> {
        if indice >= self.autos.len() {
            return Err(ConcesionariaError::IndiceInvalido(format!(
                "Índice inválido: {indice}"
            )));
        }

        self.autos.remove(indice);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn cantidad_autos(&self) -> usize {
        self.autos.len()
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

    #[test]
    fn test_agregar_y_eliminar_auto() {
        let path = archivo_temporal("concesionaria_test");
        let _ = fs::remove_file(&path);

        let mut concesionaria = Concesionaria::nuevo(2, &path).unwrap();
        let auto1 = Auto {
            marca: "Ford".to_string(),
            modelo: "Focus".to_string(),
            anio: 2020,
        };
        let auto2 = Auto {
            marca: "Chevrolet".to_string(),
            modelo: "Onix".to_string(),
            anio: 2022,
        };

        concesionaria.agregar_auto(auto1.clone()).unwrap();
        concesionaria.agregar_auto(auto2.clone()).unwrap();
        assert_eq!(concesionaria.cantidad_autos(), 2);

        concesionaria.eliminar_auto(0).unwrap();
        assert_eq!(concesionaria.cantidad_autos(), 1);
        assert_eq!(concesionaria.autos[0], auto2);
    }

    #[test]
    fn test_limite_alcanzado() {
        let path = archivo_temporal("concesionaria_limite");
        let _ = fs::remove_file(&path);

        let mut concesionaria = Concesionaria::nuevo(1, &path).unwrap();
        let auto1 = Auto {
            marca: "Toyota".to_string(),
            modelo: "Corolla".to_string(),
            anio: 2021,
        };
        let auto2 = Auto {
            marca: "Honda".to_string(),
            modelo: "Civic".to_string(),
            anio: 2023,
        };

        concesionaria.agregar_auto(auto1).unwrap();
        let error = concesionaria.agregar_auto(auto2).unwrap_err();
        assert!(matches!(error, ConcesionariaError::LimiteAlcanzado(_)));
    }
}
