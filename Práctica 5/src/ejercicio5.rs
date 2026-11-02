use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoSuscripcion {
    Basica,
    Clasica,
    Super,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MedioPago {
    Efectivo,
    MercadoPago,
    TarjetaDeCredito,
    TransferenciaBancaria,
    Cripto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suscripcion {
    pub tipo: TipoSuscripcion,
    pub costo_mensual: f64,
    pub duracion_meses: u32,
    pub fecha_inicio: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usuario {
    pub nombre: String,
    pub suscripcion: Option<Suscripcion>,
    pub medio_pago: Option<MedioPago>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SistemaStreaming {
    pub usuarios: Vec<Usuario>,
    pub historial_pagos: Vec<MedioPago>,
    pub historial_suscripciones: Vec<TipoSuscripcion>,
    #[serde(skip)]
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SistemaStreamingError {
    Archivo(String),
    IndiceInvalido(String),
}

impl SistemaStreaming {
    pub fn nuevo(archivo: impl Into<PathBuf>) -> Result<Self, SistemaStreamingError> {
        let mut sistema = Self {
            usuarios: Vec::new(),
            historial_pagos: Vec::new(),
            historial_suscripciones: Vec::new(),
            archivo: archivo.into(),
        };
        sistema.cargar_desde_archivo()?;
        Ok(sistema)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), SistemaStreamingError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            SistemaStreamingError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.usuarios = Vec::new();
            self.historial_pagos = Vec::new();
            self.historial_suscripciones = Vec::new();
            return Ok(());
        }

        let sistema_guardado: SistemaStreaming = serde_json::from_str(&contenido).map_err(|e| {
            SistemaStreamingError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;

        self.usuarios = sistema_guardado.usuarios;
        self.historial_pagos = sistema_guardado.historial_pagos;
        self.historial_suscripciones = sistema_guardado.historial_suscripciones;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), SistemaStreamingError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    SistemaStreamingError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(self).map_err(|e| {
            SistemaStreamingError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            SistemaStreamingError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn crear_usuario(&mut self, usuario: Usuario) -> Result<(), SistemaStreamingError> {
        if let Some(medio_pago) = usuario.medio_pago {
            self.historial_pagos.push(medio_pago);
        }
        if let Some(suscripcion) = &usuario.suscripcion {
            self.historial_suscripciones.push(suscripcion.tipo);
        }
        self.usuarios.push(usuario);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn mejorar_suscripcion(&mut self, indice: usize) -> Result<(), SistemaStreamingError> {
        let usuario = self.usuarios.get_mut(indice).ok_or_else(|| {
            SistemaStreamingError::IndiceInvalido(format!("Índice inválido: {indice}"))
        })?;

        if let Some(suscripcion) = usuario.suscripcion.as_mut() {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Basica => TipoSuscripcion::Clasica,
                TipoSuscripcion::Clasica => TipoSuscripcion::Super,
                TipoSuscripcion::Super => TipoSuscripcion::Super,
            };
            self.historial_suscripciones.push(suscripcion.tipo);
            self.guardar_en_archivo()?;
        }
        Ok(())
    }

    pub fn empeorar_suscripcion(&mut self, indice: usize) -> Result<(), SistemaStreamingError> {
        let usuario = self.usuarios.get_mut(indice).ok_or_else(|| {
            SistemaStreamingError::IndiceInvalido(format!("Índice inválido: {indice}"))
        })?;

        if let Some(suscripcion) = usuario.suscripcion.as_mut() {
            suscripcion.tipo = match suscripcion.tipo {
                TipoSuscripcion::Basica => TipoSuscripcion::Basica,
                TipoSuscripcion::Clasica => TipoSuscripcion::Basica,
                TipoSuscripcion::Super => TipoSuscripcion::Clasica,
            };
            if suscripcion.tipo == TipoSuscripcion::Basica {
                usuario.suscripcion = None;
            } else {
                self.historial_suscripciones.push(suscripcion.tipo);
            }
            self.guardar_en_archivo()?;
        }
        Ok(())
    }

    pub fn cancelar_suscripcion(&mut self, indice: usize) -> Result<(), SistemaStreamingError> {
        let usuario = self.usuarios.get_mut(indice).ok_or_else(|| {
            SistemaStreamingError::IndiceInvalido(format!("Índice inválido: {indice}"))
        })?;
        usuario.suscripcion = None;
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn medio_pago_mas_usado_activo(&self) -> Option<MedioPago> {
        let mut conteos = Vec::new();
        for usuario in &self.usuarios {
            if let Some(medio_pago) = usuario.medio_pago {
                if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == medio_pago) {
                    *conteo += 1;
                } else {
                    conteos.push((medio_pago, 1));
                }
            }
        }

        let mut mejor_medio = None;
        let mut mejor_conteo = 0usize;
        for (medio_pago, conteo) in conteos {
            if conteo > mejor_conteo || (conteo == mejor_conteo && mejor_medio.is_some()) {
                mejor_medio = Some(medio_pago);
                mejor_conteo = conteo;
            }
        }
        mejor_medio
    }

    pub fn suscripcion_mas_contratada_activa(&self) -> Option<TipoSuscripcion> {
        let mut conteos = Vec::new();
        for usuario in &self.usuarios {
            if let Some(suscripcion) = &usuario.suscripcion {
                if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == suscripcion.tipo) {
                    *conteo += 1;
                } else {
                    conteos.push((suscripcion.tipo, 1));
                }
            }
        }

        let mut mejor_tipo = None;
        let mut mejor_conteo = 0usize;
        for (tipo, conteo) in conteos {
            if conteo > mejor_conteo {
                mejor_tipo = Some(tipo);
                mejor_conteo = conteo;
            }
        }
        mejor_tipo
    }

    pub fn medio_pago_mas_usado_historico(&self) -> Option<MedioPago> {
        let mut conteos = Vec::new();
        for medio_pago in &self.historial_pagos {
            if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == *medio_pago) {
                *conteo += 1;
            } else {
                conteos.push((*medio_pago, 1));
            }
        }

        let mut mejor_medio = None;
        let mut mejor_conteo = 0usize;
        for (medio_pago, conteo) in conteos {
            if conteo > mejor_conteo {
                mejor_medio = Some(medio_pago);
                mejor_conteo = conteo;
            }
        }
        mejor_medio
    }

    pub fn suscripcion_mas_contratada_historica(&self) -> Option<TipoSuscripcion> {
        let mut conteos = Vec::new();
        for tipo in &self.historial_suscripciones {
            if let Some((_, conteo)) = conteos.iter_mut().find(|(item, _)| *item == *tipo) {
                *conteo += 1;
            } else {
                conteos.push((*tipo, 1));
            }
        }

        let mut mejor_tipo = None;
        let mut mejor_conteo = 0usize;
        for (tipo, conteo) in conteos {
            if conteo > mejor_conteo {
                mejor_tipo = Some(tipo);
                mejor_conteo = conteo;
            }
        }
        mejor_tipo
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
    fn test_sistema_streaming() {
        let path = archivo_temporal("streaming_test");
        let _ = fs::remove_file(&path);

        let mut sistema = SistemaStreaming::nuevo(&path).unwrap();
        sistema
            .crear_usuario(Usuario {
                nombre: "Ana".to_string(),
                suscripcion: Some(Suscripcion {
                    tipo: TipoSuscripcion::Basica,
                    costo_mensual: 1000.0,
                    duracion_meses: 1,
                    fecha_inicio: "2026-01-01".to_string(),
                }),
                medio_pago: Some(MedioPago::MercadoPago),
            })
            .unwrap();
        sistema
            .crear_usuario(Usuario {
                nombre: "Luis".to_string(),
                suscripcion: Some(Suscripcion {
                    tipo: TipoSuscripcion::Clasica,
                    costo_mensual: 1500.0,
                    duracion_meses: 3,
                    fecha_inicio: "2026-01-05".to_string(),
                }),
                medio_pago: Some(MedioPago::TarjetaDeCredito),
            })
            .unwrap();
        sistema
            .crear_usuario(Usuario {
                nombre: "Marta".to_string(),
                suscripcion: None,
                medio_pago: Some(MedioPago::Efectivo),
            })
            .unwrap();

        sistema.mejorar_suscripcion(0).unwrap();
        sistema.mejorar_suscripcion(1).unwrap();
        sistema.empeorar_suscripcion(0).unwrap();
        sistema.cancelar_suscripcion(1).unwrap();

        assert_eq!(sistema.medio_pago_mas_usado_activo(), Some(MedioPago::Efectivo));
        assert_eq!(sistema.suscripcion_mas_contratada_activa(), None);
        assert_eq!(sistema.medio_pago_mas_usado_historico(), Some(MedioPago::MercadoPago));
        assert_eq!(sistema.suscripcion_mas_contratada_historica(), Some(TipoSuscripcion::Clasica));
    }
}
