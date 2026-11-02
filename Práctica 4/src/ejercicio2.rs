#[derive(Debug, Clone, PartialEq)]
pub struct Persona<'a> {
    pub nombre: &'a str,
    pub apellido: &'a str,
    pub direccion: &'a str,
    pub ciudad: &'a str,
    pub salario: f64,
    pub edad: u8,
}

impl<'a> Persona<'a> {
    pub fn new(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Self {
        Self {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}

pub fn personas_con_salario_mayor<'a>(
    personas: &'a [Persona<'a>],
    salario: f64,
) -> Vec<&'a Persona<'a>> {
    personas.iter().filter(|persona| persona.salario > salario).collect()
}

pub fn personas_mayores_y_en_ciudad<'a>(
    personas: &'a [Persona<'a>],
    edad: u8,
    ciudad: &'a str,
) -> Vec<&'a Persona<'a>> {
    personas
        .iter()
        .filter(|persona| persona.edad > edad && persona.ciudad == ciudad)
        .collect()
}

pub fn todas_en_ciudad(personas: &[Persona<'_>], ciudad: &str) -> bool {
    personas.iter().all(|persona| persona.ciudad == ciudad)
}

pub fn alguna_en_ciudad(personas: &[Persona<'_>], ciudad: &str) -> bool {
    personas.iter().any(|persona| persona.ciudad == ciudad)
}

pub fn existe_persona(personas: &[Persona<'_>], persona: &Persona<'_>) -> bool {
    personas.iter().any(|item| item == persona)
}

pub fn edades(personas: &[Persona<'_>]) -> Vec<u8> {
    personas.iter().map(|persona| persona.edad).collect()
}

pub fn menor_y_mayor_salario<'a>(personas: &'a [Persona<'a>]) -> Option<(&'a Persona<'a>, &'a Persona<'a>)> {
    if personas.is_empty() {
        return None;
    }

    let (menor, mayor) = personas.iter().skip(1).fold((&personas[0], &personas[0]), |(menor_acc, mayor_acc), persona| {
        let menor = if persona.salario < menor_acc.salario
            || (persona.salario == menor_acc.salario && persona.edad > menor_acc.edad)
        {
            persona
        } else {
            menor_acc
        };

        let mayor = if persona.salario > mayor_acc.salario
            || (persona.salario == mayor_acc.salario && persona.edad > mayor_acc.edad)
        {
            persona
        } else {
            mayor_acc
        };

        (menor, mayor)
    });

    Some((menor, mayor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funciones_personas() {
        let personas = vec![
            Persona::new("Ana", "García", "Calle 1", "La Plata", 100_000.0, 30),
            Persona::new("Luis", "Pérez", "Calle 2", "La Plata", 80_000.0, 25),
            Persona::new("Marta", "López", "Calle 3", "Bahía Blanca", 120_000.0, 35),
        ];

        let mayores = personas_con_salario_mayor(&personas, 90_000.0);
        assert_eq!(mayores.len(), 2);
        assert_eq!(mayores[0].nombre, "Ana");

        let filtradas = personas_mayores_y_en_ciudad(&personas, 28, "La Plata");
        assert_eq!(filtradas.len(), 1);
        assert_eq!(filtradas[0].nombre, "Ana");

        assert!(todas_en_ciudad(&personas, "La Plata") == false);
        assert!(alguna_en_ciudad(&personas, "La Plata") == true);
        assert!(existe_persona(&personas, &personas[1]) == true);
        assert_eq!(edades(&personas), vec![30, 25, 35]);
        let (menor, mayor) = menor_y_mayor_salario(&personas).unwrap();
        assert_eq!(menor.nombre, "Luis");
        assert_eq!(mayor.nombre, "Marta");
    }
}
