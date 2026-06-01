/*Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección (que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢  new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢  to_string: que retorna un string con  los datos de la persona concatenados sobre el mensaje ejecutado por ej:
person.to_string(), donde person es una variable del tipo Persona.
➢  obtener_edad: retorna la edad de la persona.
➢  actualizar_direccion(nueva_direccion) */

#[derive(Debug, Clone)]
pub struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Self {
        Self {
            nombre,
            edad,
            direccion,
        }
    }

    pub fn to_string(&self) -> String {
        let direccion_str = match &self.direccion {
            Some(direccion) => direccion.as_str(),
            None => "Dirección no proporcionada",
        };
        format!(
            "Nombre: {}, Edad: {}, Dirección: {}",
            self.nombre, self.edad, direccion_str
        )
    }

    pub fn obtener_edad(&self) -> u32 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = Some(nueva_direccion);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_new_con_direccion() {
        let persona = Persona::new("Ana".to_string(), 30, Some("Calle 1".to_string()));

        assert_eq!(persona.nombre, "Ana");
        assert_eq!(persona.edad, 30);
        assert_eq!(persona.direccion, Some("Calle 1".to_string()));
    }

    #[test]
    fn test_persona_new_sin_direccion() {
        let persona = Persona::new("Ana".to_string(), 30, None);

        assert_eq!(persona.direccion, None);
    }

    #[test]
    fn test_persona_to_string_con_direccion() {
        let persona = Persona::new("Ana".to_string(), 30, Some("Calle 1".to_string()));

        assert_eq!(
            persona.to_string(),
            "Nombre: Ana, Edad: 30, Dirección: Calle 1"
        );
    }

    #[test]
    fn test_persona_to_string_sin_direccion() {
        let persona = Persona::new("Ana".to_string(), 30, None);

        assert_eq!(
            persona.to_string(),
            "Nombre: Ana, Edad: 30, Dirección: Dirección no proporcionada"
        );
    }

    #[test]
    fn test_persona_obtener_edad() {
        let persona = Persona::new("Ana".to_string(), 30, None);

        assert_eq!(persona.obtener_edad(), 30);
    }

    #[test]
    fn test_persona_actualizar_direccion() {
        let mut persona = Persona::new("Ana".to_string(), 30, None);

        persona.actualizar_direccion("Calle 2".to_string());

        assert_eq!(persona.direccion, Some("Calle 2".to_string()));
    }
}
