/* Defina la estructura Cancion con campos para el título, el artista y el género. El género 
puede ser rock, pop, rap,  jazz, otros. Luego modele una playlist. La playlist está compuesta 
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre ella: 
➔  Agregar canción. 
➔  Eliminar canción. 
➔  Mover canción (mueve la canción a una determinada posición de la playlist). 
➔  Buscar canción por nombre. 
➔  Obtener las canciones de un determinado género. 
➔  Obtener las canciones de un determinado artista. 
➔  Modificar título de la playlist. 
➔  Eliminar todas las canciones.*/

#[derive(Debug, Clone)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, Clone)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

#[derive(Debug)]
pub struct Playlist {
    pub nombre: String,
    pub canciones: Vec<Cancion>,
}

impl Playlist {
    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, titulo: &str) {
        self.canciones.retain(|c| c.titulo != titulo);
    }

    pub fn mover_cancion(&mut self, titulo: &str, nueva_posicion: usize) {
        if let Some(pos) = self.canciones.iter().position(|c| c.titulo == titulo) {
            let cancion = self.canciones.remove(pos);
            self.canciones.insert(nueva_posicion, cancion);
        }
    }

    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|c| c.titulo == titulo)
    }

    pub fn obtener_canciones_por_genero(&self, genero: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| matches!((&c.genero, &genero),
                (Genero::Rock, Genero::Rock)
                    | (Genero::Pop, Genero::Pop)
                    | (Genero::Rap, Genero::Rap)
                    | (Genero::Jazz, Genero::Jazz)
                    | (Genero::Otros, Genero::Otros)
            ))
            .collect()
    }

    pub fn obtener_canciones_por_artista(&self, artista: &str) -> Vec<&Cancion> {
        self.canciones.iter().filter(|c| c.artista == artista).collect()
    }

    pub fn modificar_titulo_playlist(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
    }

    pub fn eliminar_todas_las_canciones(&mut self) {
        self.canciones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_playlist_de_prueba() -> Playlist {
        Playlist {
            nombre: "Favoritas".to_string(),
            canciones: vec![
                Cancion {
                    titulo: "Tema 1".to_string(),
                    artista: "Artista A".to_string(),
                    genero: Genero::Rock,
                },
                Cancion {
                    titulo: "Tema 2".to_string(),
                    artista: "Artista B".to_string(),
                    genero: Genero::Pop,
                },
                Cancion {
                    titulo: "Tema 3".to_string(),
                    artista: "Artista A".to_string(),
                    genero: Genero::Rock,
                },
            ],
        }
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist {
            nombre: "Nueva".to_string(),
            canciones: Vec::new(),
        };
        let cancion = Cancion {
            titulo: "Cancion X".to_string(),
            artista: "Artista X".to_string(),
            genero: Genero::Jazz,
        };

        playlist.agregar_cancion(cancion);

        assert_eq!(playlist.canciones.len(), 1);
        assert_eq!(playlist.canciones[0].titulo, "Cancion X");
    }

    #[test]
    fn test_eliminar_cancion() {
        let mut playlist = crear_playlist_de_prueba();

        playlist.eliminar_cancion("Tema 2");

        assert_eq!(playlist.canciones.len(), 2);
        assert!(playlist.buscar_cancion_por_nombre("Tema 2").is_none());
    }

    #[test]
    fn test_mover_cancion() {
        let mut playlist = crear_playlist_de_prueba();

        playlist.mover_cancion("Tema 1", 2);

        assert_eq!(playlist.canciones[2].titulo, "Tema 1");
        assert_eq!(playlist.canciones[0].titulo, "Tema 2");
    }

    #[test]
    fn test_buscar_cancion_por_nombre() {
        let playlist = crear_playlist_de_prueba();

        let cancion = playlist.buscar_cancion_por_nombre("Tema 3");

        assert!(cancion.is_some());

        if let Some(c) = cancion {
            assert_eq!(c.artista, "Artista A");
            assert!(matches!(c.genero, Genero::Rock));
        }
    }

    #[test]
    fn test_obtener_canciones_por_genero() {
        let playlist = crear_playlist_de_prueba();

        let canciones_rock = playlist.obtener_canciones_por_genero(Genero::Rock);

        assert_eq!(canciones_rock.len(), 2);
        assert_eq!(canciones_rock[0].titulo, "Tema 1");
        assert_eq!(canciones_rock[1].titulo, "Tema 3");
    }

    #[test]
    fn test_obtener_canciones_por_artista() {
        let playlist = crear_playlist_de_prueba();

        let canciones = playlist.obtener_canciones_por_artista("Artista A");

        assert_eq!(canciones.len(), 2);
        assert_eq!(canciones[0].titulo, "Tema 1");
        assert_eq!(canciones[1].titulo, "Tema 3");
    }

    #[test]
    fn test_modificar_titulo_playlist() {
        let mut playlist = crear_playlist_de_prueba();

        playlist.modificar_titulo_playlist("Nuevas favoritas".to_string());

        assert_eq!(playlist.nombre, "Nuevas favoritas");
    }

    #[test]
    fn test_eliminar_todas_las_canciones() {
        let mut playlist = crear_playlist_de_prueba();

        playlist.eliminar_todas_las_canciones();

        assert!(playlist.canciones.is_empty());
    }
}
