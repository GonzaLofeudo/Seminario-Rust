/*Dada una cadena de veterinarias se desea implementar un sistema de atención de 
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id. 
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota 
se conoce el nombre, la edad, el tipo de animal (perro, gato, caballo, otros) y su dueño. Del 
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se 
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el 
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere. 
Dado todo lo mencionado anteriormente implemente los métodos para realizar las 
siguientes acciones: 
➔  Crear una veterinaria. 
➔  Agregar una nueva mascota a la cola de atención de la veterinaria. 
➔  Agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad. 
➔  Atender la próxima mascota de la cola. 
➔  Eliminar una mascota específica de la cola de atención dado que se retira. 
➔  Registrar una atención. 
➔  Buscar una atención dado el nombre de la mascota, el nombre del dueño y el teléfono. 
➔  Modificar el diagnóstico de una determinada atención. 
➔  Modificar la fecha de la próxima visita de una determinada atención. 
➔  Eliminar una determinada atención. 
Nota: para la fecha utilice lo implementado en el punto 3.*/

use std::collections::VecDeque;

use crate::ejercicio3::Fecha;

#[derive(Debug, Clone)]
pub struct Duenio {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
}

#[derive(Debug, Clone)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u8,
    pub tipo: TipoAnimal,
    pub dueño: Duenio,
}

#[derive(Debug, Clone)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub fecha_proxima_visita: Option<Fecha>,
}

#[derive(Debug, Clone)]
pub struct Veterinaria {
    pub id: u32,
    pub nombre: String,
    pub direccion: String,
    pub cola_atencion: VecDeque<Mascota>,
    pub registros_atencion: VecDeque<Atencion>,
}

impl Veterinaria {
    pub fn nueva_veterinaria(id: u32, nombre: String, direccion: String) -> Self {
        Veterinaria {
            id,
            nombre,
            direccion,
            cola_atencion: VecDeque::new(),
            registros_atencion: VecDeque::new(),
        }
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    pub fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota(&mut self, nombre: &str) {
        self.cola_atencion.retain(|m| m.nombre != nombre);
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registros_atencion.push_back(atencion);
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str) -> Option<&Atencion> {
        self.registros_atencion.iter().find(|a| a.mascota.nombre == nombre_mascota && a.mascota.dueño.nombre == nombre_duenio && a.mascota.dueño.telefono == telefono)
    }

    pub fn modificar_diagnostico(&mut self, nombre_mascota: &str, nuevo_diagnostico: String) {
        if let Some(atencion) = self.registros_atencion.iter_mut().find(|a| a.mascota.nombre == nombre_mascota) {
            atencion.diagnostico = nuevo_diagnostico;
        }
    }

    pub fn modificar_fecha_proxima_visita(&mut self, nombre_mascota: &str, nueva_fecha: Fecha) {
        if let Some(atencion) = self.registros_atencion.iter_mut().find(|a| a.mascota.nombre == nombre_mascota) {
            atencion.fecha_proxima_visita = Some(nueva_fecha);
        }
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str) {
        self.registros_atencion.retain(|a| a.mascota.nombre != nombre_mascota);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            dueño: crear_duenio(),
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
    fn test_nueva_veterinaria() {
        let veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );

        assert_eq!(veterinaria.id, 1);
        assert_eq!(veterinaria.nombre, "Vet Central");
        assert_eq!(veterinaria.direccion, "Av. Siempreviva");
        assert!(veterinaria.cola_atencion.is_empty());
        assert!(veterinaria.registros_atencion.is_empty());
    }

    #[test]
    fn test_agregar_y_atender_mascota() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );
        let mascota = crear_mascota("Luna");

        veterinaria.agregar_mascota(mascota);

        let atendida = veterinaria.atender_proxima_mascota();

        assert!(atendida.is_some());
        assert_eq!(atendida.unwrap().nombre, "Luna");
        assert!(veterinaria.cola_atencion.is_empty());
    }

    #[test]
    fn test_agregar_mascota_prioritaria() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );

        veterinaria.agregar_mascota(crear_mascota("Luna"));
        veterinaria.agregar_mascota_prioritaria(crear_mascota("Toby"));

        assert_eq!(veterinaria.atender_proxima_mascota().unwrap().nombre, "Toby");
        assert_eq!(veterinaria.atender_proxima_mascota().unwrap().nombre, "Luna");
    }

    #[test]
    fn test_eliminar_mascota() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );

        veterinaria.agregar_mascota(crear_mascota("Luna"));
        veterinaria.agregar_mascota(crear_mascota("Toby"));

        veterinaria.eliminar_mascota("Luna");

        assert_eq!(veterinaria.cola_atencion.len(), 1);
        assert_eq!(veterinaria.cola_atencion[0].nombre, "Toby");
    }

    #[test]
    fn test_registrar_y_buscar_atencion() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );
        let atencion = crear_atencion("Luna");

        veterinaria.registrar_atencion(atencion);

        let encontrada = veterinaria.buscar_atencion("Luna", "Ana", "123456");

        assert!(encontrada.is_some());
        if let Some(atencion_encontrada) = encontrada {
            assert_eq!(atencion_encontrada.diagnostico, "Control");
            assert_eq!(atencion_encontrada.tratamiento, "Reposo");
        }
    }

    #[test]
    fn test_modificar_diagnostico() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );
        let atencion = crear_atencion("Luna");

        veterinaria.registrar_atencion(atencion);
        veterinaria.modificar_diagnostico("Luna", "Alta".to_string());

        assert_eq!(
            veterinaria.buscar_atencion("Luna", "Ana", "123456").unwrap().diagnostico,
            "Alta"
        );
    }

    #[test]
    fn test_modificar_fecha_proxima_visita() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );
        let atencion = crear_atencion("Luna");
        let fecha = Fecha::new(10, 6, 2024);

        veterinaria.registrar_atencion(atencion);
        veterinaria.modificar_fecha_proxima_visita("Luna", fecha);

        let fecha_guardada = veterinaria
            .buscar_atencion("Luna", "Ana", "123456")
            .unwrap()
            .fecha_proxima_visita
            .as_ref()
            .unwrap();

        let fecha_esperada = Fecha::new(10, 6, 2024);

        assert!(!fecha_guardada.es_mayor(&fecha_esperada));
        assert!(!fecha_esperada.es_mayor(fecha_guardada));
    }

    #[test]
    fn test_eliminar_atencion() {
        let mut veterinaria = Veterinaria::nueva_veterinaria(
            1,
            "Vet Central".to_string(),
            "Av. Siempreviva".to_string(),
        );

        veterinaria.registrar_atencion(crear_atencion("Luna"));
        veterinaria.registrar_atencion(crear_atencion("Toby"));

        veterinaria.eliminar_atencion("Luna");

        assert_eq!(veterinaria.registros_atencion.len(), 1);
        assert_eq!(veterinaria.registros_atencion[0].mascota.nombre, "Toby");
    }
}
