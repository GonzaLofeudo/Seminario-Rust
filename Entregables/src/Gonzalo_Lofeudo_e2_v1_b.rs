#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoSuscripcion {
    Básica,
    Clásica,
    Súper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MedioPago {
    Efectivo,
    MercadoPago,
    TarjetaDeCrédito,
    TransferenciaBancaria,
    Cripto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Suscripcion {
    pub tipo: TipoSuscripcion,
    pub costoMensual: f64,
    pub duracionMeses: u32,
    pub fechaInicio: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    pub nombre: String,
    pub suscripcion: Option<Suscripcion>,
    pub medioPago: Option<MedioPago>,
}

pub struct EstacionTop {
    pub estacion: String,
    pub cantidadSuscripciones: usize,
    pub mesMasContratado: String,
    pub cantidadMesMasContratado: usize,
}

impl EstacionTop {
    pub fn new(
        estacion: String,
        cantidadSuscripciones: usize,
        mesMasContratado: String,
        cantidadMesMasContratado: usize,
    ) -> Self {
        Self {
            estacion,
            cantidadSuscripciones,
            mesMasContratado,
            cantidadMesMasContratado,
        }
    }
}

impl Usuario {
    pub fn nuevo(nombre: String, suscripcion: Option<Suscripcion>, medioPago: Option<MedioPago>) -> Self {
        Self {
            nombre,
            suscripcion,
            medioPago,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SistemaStreaming {
    pub usuarios: Vec<Usuario>,
    historialPagos: Vec<MedioPago>,
    historialSuscripciones: Vec<TipoSuscripcion>,
}

impl SistemaStreaming {
    pub fn crearUsuario(&mut self, usuario: Usuario) {
        if let Some(medioPago) = usuario.medioPago {
            self.historialPagos.push(medioPago);
        }
        if let Some(suscripcion) = &usuario.suscripcion {
            self.historialSuscripciones.push(suscripcion.tipo);
        }
        self.usuarios.push(usuario);
    }

    pub fn mejorarSuscripcion(&mut self, indice: usize) {
        if let Some(usuario) = self.usuarios.get_mut(indice) {
            if let Some(suscripcion) = usuario.suscripcion.as_mut() {
                suscripcion.tipo = match suscripcion.tipo {
                    TipoSuscripcion::Básica => TipoSuscripcion::Clásica,
                    TipoSuscripcion::Clásica => TipoSuscripcion::Súper,
                    TipoSuscripcion::Súper => TipoSuscripcion::Súper,
                };
                self.historialSuscripciones.push(suscripcion.tipo);
            }
        }
    }

    pub fn empeorarSuscripcion(&mut self, indice: usize) {
        if let Some(usuario) = self.usuarios.get_mut(indice) {
            if let Some(suscripcion) = usuario.suscripcion.as_mut() {
                suscripcion.tipo = match suscripcion.tipo {
                    TipoSuscripcion::Básica => TipoSuscripcion::Básica,
                    TipoSuscripcion::Clásica => TipoSuscripcion::Básica,
                    TipoSuscripcion::Súper => TipoSuscripcion::Clásica,
                };
                if suscripcion.tipo == TipoSuscripcion::Básica {
                    usuario.suscripcion = None;
                } else {
                    self.historialSuscripciones.push(suscripcion.tipo);
                }
            }
        }
    }

    pub fn cancelarSuscripcion(&mut self, indice: usize) {
        if let Some(usuario) = self.usuarios.get_mut(indice) {
            usuario.suscripcion = None;
        }
    }

    pub fn medioPagoMasUsadoActivo(&self) -> Option<MedioPago> {
        let mut conteos: Vec<(MedioPago, usize)> = Vec::new();

        for usuario in &self.usuarios {
            if let Some(medioPago) = usuario.medioPago {
                if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == medioPago) {
                    *conteo += 1;
                } else {
                    conteos.push((medioPago, 1));
                }
            }
        }

        let mut mejorMedio = None;
        let mut mejorConteo = 0usize;

        for (medioPago, conteo) in conteos {
            if conteo > mejorConteo || (conteo == mejorConteo && mejorMedio.is_some()) {
                mejorMedio = Some(medioPago);
                mejorConteo = conteo;
            }
        }

        mejorMedio
    }

    pub fn suscripcionMasContratadaActiva(&self) -> Option<TipoSuscripcion> {
        let mut conteos: Vec<(TipoSuscripcion, usize)> = Vec::new();

        for usuario in &self.usuarios {
            if let Some(suscripcion) = &usuario.suscripcion {
                if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == suscripcion.tipo) {
                    *conteo += 1;
                } else {
                    conteos.push((suscripcion.tipo, 1));
                }
            }
        }

        let mut mejorTipo = None;
        let mut mejorConteo = 0usize;

        for (tipo, conteo) in conteos {
            if conteo > mejorConteo {
                mejorTipo = Some(tipo);
                mejorConteo = conteo;
            }
        }

        mejorTipo
    }

    pub fn medioPagoMasUsadoHistorico(&self) -> Option<MedioPago> {
        let mut conteos: Vec<(MedioPago, usize)> = Vec::new();

        for medioPago in &self.historialPagos {
            if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == *medioPago) {
                *conteo += 1;
            } else {
                conteos.push((*medioPago, 1));
            }
        }

        let mut mejorMedio = None;
        let mut mejorConteo = 0usize;

        for (medioPago, conteo) in conteos {
            if conteo > mejorConteo {
                mejorMedio = Some(medioPago);
                mejorConteo = conteo;
            }
        }

        mejorMedio
    }

    pub fn suscripcionMasContratadaHistorica(&self) -> Option<TipoSuscripcion> {
        let mut conteos: Vec<(TipoSuscripcion, usize)> = Vec::new();

        for tipo in &self.historialSuscripciones {
            if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == *tipo) {
                *conteo += 1;
            } else {
                conteos.push((*tipo, 1));
            }
        }

        let mut mejorTipo = None;
        let mut mejorConteo = 0usize;

        for (tipo, conteo) in conteos {
            if conteo > mejorConteo {
                mejorTipo = Some(tipo);
                mejorConteo = conteo;
            }
        }

        mejorTipo
    }

    pub fn estacion_con_mas_suscripciones(&self) -> Option<EstacionTop> {
        const ESTACIONES: [(&str, [&str; 3]); 4] = [
            ("Verano", ["12", "01", "02"]),
            ("Otoño", ["03", "04", "05"]),
            ("Invierno", ["06", "07", "08"]),
            ("Primavera", ["09", "10", "11"]),
        ];

        let mut mejor_estacion: Option<EstacionTop> = None;

        for (nombre_estacion, meses) in ESTACIONES {
            let mut cantidad_suscripciones = 0usize;
            let mut conteo_meses: Vec<(&str, usize)> = Vec::new();

            for usuario in &self.usuarios {
                let Some(suscripcion) = &usuario.suscripcion else {
                    continue;
                };

                let Some(mes) = extraer_mes(&suscripcion.fechaInicio) else {
                    continue;
                };

                if meses.contains(&mes) {
                    cantidad_suscripciones += 1;
                    if let Some((_, conteo)) = conteo_meses.iter_mut().find(|(item, _)| *item == mes) {
                        *conteo += 1;
                    } else {
                        conteo_meses.push((mes, 1));
                    }
                }
            }

            if cantidad_suscripciones == 0 {
                continue;
            }

            let mut mes_mas_contratado = "";
            let mut cantidad_mes_mas_contratado = 0usize;

            for (mes, conteo) in conteo_meses {
                if conteo > cantidad_mes_mas_contratado {
                    mes_mas_contratado = mes;
                    cantidad_mes_mas_contratado = conteo;
                }
            }

            let candidato = EstacionTop::new(
                nombre_estacion.to_string(),
                cantidad_suscripciones,
                mes_mas_contratado.to_string(),
                cantidad_mes_mas_contratado,
            );

            let reemplaza = match &mejor_estacion {
                None => true,
                Some(actual) => candidato.cantidadSuscripciones > actual.cantidadSuscripciones,
            };

            if reemplaza {
                mejor_estacion = Some(candidato);
            }
        }

        mejor_estacion
    }
}

fn extraer_mes(fecha: &str) -> Option<&str> {
    let mut partes = fecha.split('-');
    partes.next()?;
    partes.next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_system() {
        let mut sistema = SistemaStreaming::default();
        sistema.crearUsuario(Usuario::nuevo(
            "Ana".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Básica,
                costoMensual: 1000.0,
                duracionMeses: 1,
                fechaInicio: "2026-01-01".to_string(),
            }),
            Some(MedioPago::MercadoPago),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Luis".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Clásica,
                costoMensual: 1500.0,
                duracionMeses: 3,
                fechaInicio: "2026-01-05".to_string(),
            }),
            Some(MedioPago::TarjetaDeCrédito),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Marta".to_string(),
            None,
            Some(MedioPago::Efectivo),
        ));

        sistema.mejorarSuscripcion(0);
        sistema.mejorarSuscripcion(1);
        sistema.empeorarSuscripcion(0);
        sistema.cancelarSuscripcion(1);

        assert_eq!(sistema.medioPagoMasUsadoActivo(), Some(MedioPago::Efectivo));
        assert_eq!(sistema.suscripcionMasContratadaActiva(), None);
        assert_eq!(sistema.medioPagoMasUsadoHistorico(), Some(MedioPago::MercadoPago));
        assert_eq!(sistema.suscripcionMasContratadaHistorica(), Some(TipoSuscripcion::Clásica));
    }

    #[test]
    fn test_estacion_con_mas_suscripciones() {
        let mut sistema = SistemaStreaming::default();

        sistema.crearUsuario(Usuario::nuevo(
            "Ana".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Básica,
                costoMensual: 1000.0,
                duracionMeses: 1,
                fechaInicio: "2026-01-01".to_string(),
            }),
            Some(MedioPago::MercadoPago),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Luis".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Clásica,
                costoMensual: 1500.0,
                duracionMeses: 3,
                fechaInicio: "2026-02-10".to_string(),
            }),
            Some(MedioPago::TarjetaDeCrédito),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Marta".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Súper,
                costoMensual: 2000.0,
                duracionMeses: 6,
                fechaInicio: "2026-07-01".to_string(),
            }),
            Some(MedioPago::Efectivo),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Nico".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Básica,
                costoMensual: 1000.0,
                duracionMeses: 1,
                fechaInicio: "2026-08-15".to_string(),
            }),
            Some(MedioPago::Cripto),
        ));

        let estacion_top = sistema.estacion_con_mas_suscripciones().unwrap();

        assert_eq!(estacion_top.estacion, "Verano");
        assert_eq!(estacion_top.cantidadSuscripciones, 2);
        assert_eq!(estacion_top.mesMasContratado, "01");
        assert_eq!(estacion_top.cantidadMesMasContratado, 1);
    }

    #[test]
    fn test_estacion_sin_suscripciones() {
        let sistema = SistemaStreaming::default();

        assert!(sistema.estacion_con_mas_suscripciones().is_none());
    }

    #[test]
    fn test_estacion_empate_en_mes() {
        let mut sistema = SistemaStreaming::default();

        sistema.crearUsuario(Usuario::nuevo(
            "Ana".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Básica,
                costoMensual: 1000.0,
                duracionMeses: 1,
                fechaInicio: "2026-01-01".to_string(),
            }),
            Some(MedioPago::MercadoPago),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Luis".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Clásica,
                costoMensual: 1500.0,
                duracionMeses: 3,
                fechaInicio: "2026-02-10".to_string(),
            }),
            Some(MedioPago::TarjetaDeCrédito),
        ));

        let estacion_top = sistema.estacion_con_mas_suscripciones().unwrap();

        assert_eq!(estacion_top.estacion, "Verano");
        assert_eq!(estacion_top.cantidadSuscripciones, 2);
        assert_eq!(estacion_top.mesMasContratado, "01");
        assert_eq!(estacion_top.cantidadMesMasContratado, 1);
    }

    #[test]
    fn test_estacion_ignora_fecha_invalida() {
        let mut sistema = SistemaStreaming::default();

        sistema.crearUsuario(Usuario::nuevo(
            "Ana".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Básica,
                costoMensual: 1000.0,
                duracionMeses: 1,
                fechaInicio: "2026-01-01".to_string(),
            }),
            Some(MedioPago::MercadoPago),
        ));
        sistema.crearUsuario(Usuario::nuevo(
            "Luis".to_string(),
            Some(Suscripcion {
                tipo: TipoSuscripcion::Clásica,
                costoMensual: 1500.0,
                duracionMeses: 3,
                fechaInicio: "2026/13/01".to_string(),
            }),
            Some(MedioPago::TarjetaDeCrédito),
        ));

        let estacion_top = sistema.estacion_con_mas_suscripciones().unwrap();

        assert_eq!(estacion_top.estacion, "Verano");
        assert_eq!(estacion_top.cantidadSuscripciones, 1);
        assert_eq!(estacion_top.mesMasContratado, "01");
        assert_eq!(estacion_top.cantidadMesMasContratado, 1);
    }
}
