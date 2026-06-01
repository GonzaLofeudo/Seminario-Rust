/*Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢es_fecha_valida: retorna true si es una fecha válida, false caso contrario. Tenga en cuenta los años bisiestos también.
➢es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢sumar_dias(dias): suma la cantidad de días a la fecha, modificándose.
➢restar_dias(dias): resta la cantidad de días a la fecha, modificándose.
➢es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a la fecha pasada por parámetro. */

#[derive(Debug, Clone)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: u32) -> Self {
        Fecha { dia, mes, anio }
    }

    fn dias_en_mes(mes: u32, anio: u32) -> u32 {
        match mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (Fecha { dia: 1, mes, anio }).es_bisiesto() {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    pub fn es_bisiesto(&self) -> bool {
        (self.anio % 4 == 0 && self.anio % 100 != 0) || (self.anio % 400 == 0)
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        let dias_en_mes = Fecha::dias_en_mes(self.mes, self.anio);

        self.dia >= 1 && self.dia <= dias_en_mes
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        if !self.es_fecha_valida() {
            panic!("La fecha debe ser valida");
        }

        self.dia += dias;
        let mut dias_en_mes = Fecha::dias_en_mes(self.mes, self.anio);

        while self.dia > dias_en_mes {
            self.dia -= dias_en_mes;
            self.mes += 1;
            if self.mes > 12 {
                self.mes = 1;
                self.anio += 1;
            }
            dias_en_mes = Fecha::dias_en_mes(self.mes, self.anio);
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        if !self.es_fecha_valida() {
            panic!("La fecha debe ser valida");
        }

        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if self.dia > 1 {
                self.dia -= 1;
            } else {
                if self.mes == 1 {
                    if self.anio == 0 {
                        panic!("No se puede restar dias antes del año 0");
                    }
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }

                self.dia = Fecha::dias_en_mes(self.mes, self.anio);
            }

            dias_restantes -= 1;
        }
    }

    pub fn es_mayor(&self, otra_fecha: &Fecha) -> bool {
        if self.anio != otra_fecha.anio {
            return self.anio > otra_fecha.anio;
        }
        if self.mes != otra_fecha.mes {
            return self.mes > otra_fecha.mes;
        }
        self.dia > otra_fecha.dia
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha_new() {
        let fecha = Fecha::new(15, 5, 2026);

        assert_eq!(fecha.dia, 15);
        assert_eq!(fecha.mes, 5);
        assert_eq!(fecha.anio, 2026);
    }

    #[test]
    fn test_fecha_valida() {
        let fecha = Fecha::new(31, 12, 2026);

        assert_eq!(fecha.es_fecha_valida(), true);
    }

    #[test]
    fn test_fecha_invalida_por_mes() {
        let fecha = Fecha::new(10, 13, 2026);

        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn test_fecha_invalida_por_dia() {
        let fecha = Fecha::new(31, 4, 2026);

        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn test_fecha_valida_29_febrero_bisiesto() {
        let fecha = Fecha::new(29, 2, 2024);

        assert_eq!(fecha.es_fecha_valida(), true);
    }

    #[test]
    fn test_fecha_invalida_29_febrero_no_bisiesto() {
        let fecha = Fecha::new(29, 2, 2023);

        assert_eq!(fecha.es_fecha_valida(), false);
    }

    #[test]
    fn test_es_bisiesto_divisible_por_400() {
        let fecha = Fecha::new(1, 1, 2000);

        assert_eq!(fecha.es_bisiesto(), true);
    }

    #[test]
    fn test_es_bisiesto_divisible_por_100_no_por_400() {
        let fecha = Fecha::new(1, 1, 1900);

        assert_eq!(fecha.es_bisiesto(), false);
    }

    #[test]
    fn test_sumar_dias_mismo_mes() {
        let mut fecha = Fecha::new(10, 5, 2026);

        fecha.sumar_dias(5);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (15, 5, 2026));
    }

    #[test]
    fn test_sumar_dias_cambia_de_mes() {
        let mut fecha = Fecha::new(30, 4, 2026);

        fecha.sumar_dias(1);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (1, 5, 2026));
    }

    #[test]
    fn test_sumar_dias_cambia_de_anio() {
        let mut fecha = Fecha::new(31, 12, 2026);

        fecha.sumar_dias(1);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (1, 1, 2027));
    }

    #[test]
    fn test_restar_dias_mismo_mes() {
        let mut fecha = Fecha::new(10, 5, 2026);

        fecha.restar_dias(5);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (5, 5, 2026));
    }

    #[test]
    fn test_restar_dias_cambia_de_mes() {
        let mut fecha = Fecha::new(1, 5, 2026);

        fecha.restar_dias(1);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (30, 4, 2026));
    }

    #[test]
    fn test_restar_dias_cambia_de_anio() {
        let mut fecha = Fecha::new(1, 1, 2026);

        fecha.restar_dias(1);

        assert_eq!((fecha.dia, fecha.mes, fecha.anio), (31, 12, 2025));
    }

    #[test]
    fn test_es_mayor_true() {
        let fecha = Fecha::new(2, 1, 2026);
        let otra_fecha = Fecha::new(1, 1, 2026);

        assert_eq!(fecha.es_mayor(&otra_fecha), true);
    }

    #[test]
    fn test_es_mayor_false_si_son_iguales() {
        let fecha = Fecha::new(1, 1, 2026);
        let otra_fecha = Fecha::new(1, 1, 2026);

        assert_eq!(fecha.es_mayor(&otra_fecha), false);
    }

    #[test]
    #[should_panic(expected = "La fecha debe ser valida")]
    fn test_sumar_dias_con_fecha_invalida() {
        let mut fecha = Fecha::new(31, 4, 2026);

        fecha.sumar_dias(1);
    }
}
