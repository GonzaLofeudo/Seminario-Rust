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
}
