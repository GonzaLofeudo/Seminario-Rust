/*Escribir un programa que defina una estructura Estudiante que tenga campos para el 
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se 
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes 
métodos:
❖  Examen: 
➢  new: que pasando los parámetros correspondientes, crea un Examen y lo retorna. 
❖  Estudiante: 
➢  new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna. 
➢  obtener_promedio: retorna el promedio de las notas. 
➢  obtener_calificacion_mas_alta: retorna la nota más alta. 
➢  obtener_calificacion_mas_baja: retorna la nota más baja. 
Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen. */

#[derive(Debug, Clone)]
pub struct Examen {
    pub materia: String,
    pub nota: f64,
}

impl Examen {
    pub fn new(materia: String, nota: f64) -> Self {
        Examen { materia, nota }
    }
}

#[derive(Debug, Clone)]
pub struct Estudiante {
    pub nombre: String,
    pub id: u32,
    pub examenes: Vec<Examen>,
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, examenes: Vec<Examen>) -> Self {
        Estudiante { nombre, id, examenes }
    }

    pub fn obtener_promedio(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }
        let total: f64 = self.examenes.iter().map(|e| e.nota).sum();
        Some(total / self.examenes.len() as f64)
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f64> {
        self.examenes.iter().map(|e| e.nota).max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f64> {
        self.examenes.iter().map(|e| e.nota).min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examen_new() {
        let examen = Examen::new("Matematica".to_string(), 8.5);

        assert_eq!(examen.materia, "Matematica");
        assert_eq!(examen.nota, 8.5);
    }

    #[test]
    fn test_estudiante_new() {
        let examenes = vec![
            Examen::new("Matematica".to_string(), 8.5),
            Examen::new("Historia".to_string(), 7.0),
        ];
        let estudiante = Estudiante::new("Ana".to_string(), 10, examenes.clone());

        assert_eq!(estudiante.nombre, "Ana");
        assert_eq!(estudiante.id, 10);
        assert_eq!(estudiante.examenes.len(), 2);
        assert_eq!(estudiante.examenes[0].materia, examenes[0].materia);
        assert_eq!(estudiante.examenes[1].nota, examenes[1].nota);
    }

    #[test]
    fn test_obtener_promedio() {
        let examenes = vec![
            Examen::new("Matematica".to_string(), 8.0),
            Examen::new("Historia".to_string(), 6.0),
            Examen::new("Fisica".to_string(), 10.0),
        ];
        let estudiante = Estudiante::new("Ana".to_string(), 10, examenes);

        assert_eq!(estudiante.obtener_promedio(), Some(8.0));
    }

    #[test]
    fn test_obtener_calificacion_mas_alta() {
        let examenes = vec![
            Examen::new("Matematica".to_string(), 8.0),
            Examen::new("Historia".to_string(), 6.0),
            Examen::new("Fisica".to_string(), 10.0),
        ];
        let estudiante = Estudiante::new("Ana".to_string(), 10, examenes);

        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(10.0));
    }

    #[test]
    fn test_obtener_calificacion_mas_baja() {
        let examenes = vec![
            Examen::new("Matematica".to_string(), 8.0),
            Examen::new("Historia".to_string(), 6.0),
            Examen::new("Fisica".to_string(), 10.0),
        ];
        let estudiante = Estudiante::new("Ana".to_string(), 10, examenes);

        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(6.0));
    }

    #[test]
    fn test_estudiante_sin_examenes() {
        let estudiante = Estudiante::new("Ana".to_string(), 10, Vec::new());

        assert_eq!(estudiante.obtener_promedio(), None);
        assert_eq!(estudiante.obtener_calificacion_mas_alta(), None);
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), None);
    }
}
