use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    pub nombre: String,
    pub canciones: Vec<Cancion>,
    #[serde(skip)]
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistError {
    Archivo(String),
    CancionNoEncontrada(String),
    PosicionInvalida(String),
}

impl Playlist {
    pub fn nuevo(nombre: impl Into<String>, archivo: impl Into<PathBuf>) -> Result<Self, PlaylistError> {
        let mut playlist = Self {
            nombre: nombre.into(),
            canciones: Vec::new(),
            archivo: archivo.into(),
        };
        playlist.cargar_desde_archivo()?;
        Ok(playlist)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), PlaylistError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            PlaylistError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.canciones = Vec::new();
            return Ok(());
        }

        let playlist_guardada: Playlist = serde_json::from_str(&contenido).map_err(|e| {
            PlaylistError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;

        self.nombre = playlist_guardada.nombre;
        self.canciones = playlist_guardada.canciones;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), PlaylistError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    PlaylistError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(self).map_err(|e| {
            PlaylistError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            PlaylistError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) -> Result<(), PlaylistError> {
        self.canciones.push(cancion);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn eliminar_cancion(&mut self, titulo: &str) -> Result<(), PlaylistError> {
        let original = self.canciones.len();
        self.canciones.retain(|c| c.titulo != titulo);
        if self.canciones.len() == original {
            return Err(PlaylistError::CancionNoEncontrada(format!(
                "No se encontró la canción: {titulo}"
            )));
        }
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn mover_cancion(&mut self, titulo: &str, nueva_posicion: usize) -> Result<(), PlaylistError> {
        let posicion = self
            .canciones
            .iter()
            .position(|c| c.titulo == titulo)
            .ok_or_else(|| PlaylistError::CancionNoEncontrada(format!("No se encontró la canción: {titulo}")))?;

        if nueva_posicion >= self.canciones.len() {
            return Err(PlaylistError::PosicionInvalida(format!(
                "Posición inválida: {nueva_posicion}"
            )));
        }

        let cancion = self.canciones.remove(posicion);
        self.canciones.insert(nueva_posicion, cancion);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|c| c.titulo == titulo)
    }

    pub fn obtener_canciones_por_genero(&self, genero: Genero) -> Vec<&Cancion> {
        self.canciones.iter().filter(|c| c.genero == genero).collect()
    }

    pub fn obtener_canciones_por_artista(&self, artista: &str) -> Vec<&Cancion> {
        self.canciones.iter().filter(|c| c.artista == artista).collect()
    }

    pub fn modificar_titulo_playlist(&mut self, nuevo_nombre: String) -> Result<(), PlaylistError> {
        self.nombre = nuevo_nombre;
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn eliminar_todas_las_canciones(&mut self) -> Result<(), PlaylistError> {
        self.canciones.clear();
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

    #[test]
    fn test_agregar_y_cargar_desde_archivo() {
        let path = archivo_temporal("playlist_test");
        let _ = fs::remove_file(&path);

        let mut playlist = Playlist::nuevo("Favoritas", &path).unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 1".to_string(),
                artista: "Artista A".to_string(),
                genero: Genero::Rock,
            })
            .unwrap();

        let playlist_recargada = Playlist::nuevo("Favoritas", &path).unwrap();
        assert_eq!(playlist_recargada.canciones.len(), 1);
        assert_eq!(playlist_recargada.canciones[0].titulo, "Tema 1");
    }

    #[test]
    fn test_eliminar_y_mover_cancion() {
        let path = archivo_temporal("playlist_movimiento");
        let _ = fs::remove_file(&path);

        let mut playlist = Playlist::nuevo("Favoritas", &path).unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 1".to_string(),
                artista: "Artista A".to_string(),
                genero: Genero::Rock,
            })
            .unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 2".to_string(),
                artista: "Artista B".to_string(),
                genero: Genero::Pop,
            })
            .unwrap();

        playlist.eliminar_cancion("Tema 1").unwrap();
        playlist.mover_cancion("Tema 2", 0).unwrap();

        assert_eq!(playlist.canciones.len(), 1);
        assert_eq!(playlist.canciones[0].titulo, "Tema 2");
    }

    #[test]
    fn test_filtros_y_modificacion() {
        let path = archivo_temporal("playlist_filtros");
        let _ = fs::remove_file(&path);

        let mut playlist = Playlist::nuevo("Favoritas", &path).unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 1".to_string(),
                artista: "Artista A".to_string(),
                genero: Genero::Rock,
            })
            .unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 2".to_string(),
                artista: "Artista B".to_string(),
                genero: Genero::Pop,
            })
            .unwrap();
        playlist
            .agregar_cancion(Cancion {
                titulo: "Tema 3".to_string(),
                artista: "Artista A".to_string(),
                genero: Genero::Rock,
            })
            .unwrap();

        assert_eq!(playlist.buscar_cancion_por_nombre("Tema 3").unwrap().artista, "Artista A");
        let rock = playlist.obtener_canciones_por_genero(Genero::Rock);
        assert_eq!(rock.len(), 2);
        let artista_a = playlist.obtener_canciones_por_artista("Artista A");
        assert_eq!(artista_a.len(), 2);

        playlist.modificar_titulo_playlist("Nuevas favoritas".to_string()).unwrap();
        assert_eq!(playlist.nombre, "Nuevas favoritas");

        playlist.eliminar_todas_las_canciones().unwrap();
        assert!(playlist.canciones.is_empty());
    }
}
