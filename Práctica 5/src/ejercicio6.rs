use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub dni: String,
    pub validado: bool,
    pub balances_cripto: HashMap<String, f64>,
    pub balance_fiat: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaccion {
    pub fecha: String,
    pub usuario: String,
    pub tipo: String,
    pub monto: f64,
    pub cotizacion: Option<f64>,
    pub cripto: Option<String>,
    pub blockchain: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlataformaCripto {
    pub usuarios: Vec<Usuario>,
    pub transacciones: Vec<Transaccion>,
    pub cotizaciones: HashMap<String, f64>,
    #[serde(skip)]
    pub archivo: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlataformaCriptoError {
    Archivo(String),
    UsuarioNoEncontrado(String),
    FondosInsuficientes(String),
    CriptoNoDisponible(String),
}

impl PlataformaCripto {
    pub fn nuevo(archivo: impl Into<PathBuf>) -> Result<Self, PlataformaCriptoError> {
        let mut plataforma = Self {
            usuarios: Vec::new(),
            transacciones: Vec::new(),
            cotizaciones: HashMap::new(),
            archivo: archivo.into(),
        };
        plataforma.cargar_desde_archivo()?;
        Ok(plataforma)
    }

    fn cargar_desde_archivo(&mut self) -> Result<(), PlataformaCriptoError> {
        if !self.archivo.exists() {
            return Ok(());
        }

        let contenido = fs::read_to_string(&self.archivo).map_err(|e| {
            PlataformaCriptoError::Archivo(format!("No se pudo leer el archivo: {e}"))
        })?;

        if contenido.trim().is_empty() {
            self.usuarios = Vec::new();
            self.transacciones = Vec::new();
            self.cotizaciones = HashMap::new();
            return Ok(());
        }

        let plataforma_guardada: PlataformaCripto = serde_json::from_str(&contenido).map_err(|e| {
            PlataformaCriptoError::Archivo(format!("No se pudo leer JSON: {e}"))
        })?;

        self.usuarios = plataforma_guardada.usuarios;
        self.transacciones = plataforma_guardada.transacciones;
        self.cotizaciones = plataforma_guardada.cotizaciones;
        Ok(())
    }

    fn guardar_en_archivo(&self) -> Result<(), PlataformaCriptoError> {
        if let Some(directorio) = self.archivo.parent() {
            if !directorio.as_os_str().is_empty() {
                fs::create_dir_all(directorio).map_err(|e| {
                    PlataformaCriptoError::Archivo(format!("No se pudo crear el directorio: {e}"))
                })?;
            }
        }

        let contenido = serde_json::to_string_pretty(self).map_err(|e| {
            PlataformaCriptoError::Archivo(format!("No se pudo serializar: {e}"))
        })?;

        fs::write(&self.archivo, contenido).map_err(|e| {
            PlataformaCriptoError::Archivo(format!("No se pudo guardar el archivo: {e}"))
        })?;
        Ok(())
    }

    pub fn agregar_usuario(&mut self, usuario: Usuario) -> Result<(), PlataformaCriptoError> {
        self.usuarios.push(usuario);
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn ingresar_fiat(&mut self, usuario_idx: usize, monto: f64) -> Result<(), PlataformaCriptoError> {
        let usuario = self.usuarios.get_mut(usuario_idx).ok_or_else(|| {
            PlataformaCriptoError::UsuarioNoEncontrado(format!("No se encontró el usuario en la posición: {usuario_idx}"))
        })?;

        usuario.balance_fiat += monto;
        self.transacciones.push(Transaccion {
            fecha: "2026-01-01".to_string(),
            usuario: usuario.nombre.clone(),
            tipo: "ingreso de dinero".to_string(),
            monto,
            cotizacion: None,
            cripto: None,
            blockchain: None,
        });
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn comprar_cripto(&mut self, usuario_idx: usize, cripto: &str, monto_fiat: f64) -> Result<(), PlataformaCriptoError> {
        let usuario = self.usuarios.get_mut(usuario_idx).ok_or_else(|| {
            PlataformaCriptoError::UsuarioNoEncontrado(format!("No se encontró el usuario en la posición: {usuario_idx}"))
        })?;

        if !usuario.validado {
            return Ok(());
        }

        let cotizacion = self.cotizaciones.get(cripto).copied().ok_or_else(|| {
            PlataformaCriptoError::CriptoNoDisponible(format!("No hay cotización para: {cripto}"))
        })?;

        if usuario.balance_fiat < monto_fiat {
            return Err(PlataformaCriptoError::FondosInsuficientes(format!(
                "Fondos insuficientes para comprar {cripto}"
            )));
        }

        let cantidad = monto_fiat / cotizacion;
        usuario.balance_fiat -= monto_fiat;
        *usuario.balances_cripto.entry(cripto.to_string()).or_insert(0.0) += cantidad;
        self.transacciones.push(Transaccion {
            fecha: "2026-01-01".to_string(),
            usuario: usuario.nombre.clone(),
            tipo: "compra de cripto".to_string(),
            monto: cantidad,
            cotizacion: Some(cotizacion),
            cripto: Some(cripto.to_string()),
            blockchain: None,
        });
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn vender_cripto(&mut self, usuario_idx: usize, cripto: &str, monto_cripto: f64) -> Result<(), PlataformaCriptoError> {
        let usuario = self.usuarios.get_mut(usuario_idx).ok_or_else(|| {
            PlataformaCriptoError::UsuarioNoEncontrado(format!("No se encontró el usuario en la posición: {usuario_idx}"))
        })?;

        if !usuario.validado {
            return Ok(());
        }

        let cotizacion = self.cotizaciones.get(cripto).copied().ok_or_else(|| {
            PlataformaCriptoError::CriptoNoDisponible(format!("No hay cotización para: {cripto}"))
        })?;

        let disponible = usuario.balances_cripto.get(cripto).copied().unwrap_or(0.0);
        if disponible < monto_cripto {
            return Err(PlataformaCriptoError::CriptoNoDisponible(format!(
                "No tienes suficiente {cripto} para vender"
            )));
        }

        usuario.balances_cripto.insert(cripto.to_string(), disponible - monto_cripto);
        usuario.balance_fiat += monto_cripto * cotizacion;
        self.transacciones.push(Transaccion {
            fecha: "2026-01-01".to_string(),
            usuario: usuario.nombre.clone(),
            tipo: "venta de cripto".to_string(),
            monto: monto_cripto,
            cotizacion: Some(cotizacion),
            cripto: Some(cripto.to_string()),
            blockchain: None,
        });
        self.guardar_en_archivo()?;
        Ok(())
    }

    pub fn cripto_mas_compras(&self) -> Option<String> {
        let mut conteos = HashMap::new();
        for tx in &self.transacciones {
            if tx.tipo == "compra de cripto" {
                if let Some(cripto) = &tx.cripto {
                    *conteos.entry(cripto.clone()).or_insert(0usize) += 1;
                }
            }
        }
        conteos.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(cripto, _)| cripto)
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
    fn test_balances_y_transacciones_persisten() {
        let path = archivo_temporal("cripto_test");
        let _ = fs::remove_file(&path);

        let mut plataforma = PlataformaCripto::nuevo(&path).unwrap();
        plataforma
            .agregar_usuario(Usuario {
                nombre: "Ana".to_string(),
                apellido: "García".to_string(),
                email: "ana@test.com".to_string(),
                dni: "123".to_string(),
                validado: true,
                balances_cripto: HashMap::new(),
                balance_fiat: 1000.0,
            })
            .unwrap();
        plataforma.cotizaciones.insert("BTC".to_string(), 100.0);

        plataforma.ingresar_fiat(0, 500.0).unwrap();
        plataforma.comprar_cripto(0, "BTC", 200.0).unwrap();

        let plataforma_recargada = PlataformaCripto::nuevo(&path).unwrap();
        assert_eq!(plataforma_recargada.usuarios[0].balance_fiat, 1300.0);
        assert_eq!(plataforma_recargada.usuarios[0].balances_cripto["BTC"], 2.0);
        assert_eq!(plataforma_recargada.cripto_mas_compras(), Some("BTC".to_string()));
    }

    #[test]
    fn test_errores_de_saldo_y_usuario() {
        let path = archivo_temporal("cripto_errores");
        let _ = fs::remove_file(&path);

        let mut plataforma = PlataformaCripto::nuevo(&path).unwrap();
        let error_indice = plataforma.ingresar_fiat(0, 100.0).unwrap_err();
        assert!(matches!(error_indice, PlataformaCriptoError::UsuarioNoEncontrado(_)));

        plataforma
            .agregar_usuario(Usuario {
                nombre: "Luis".to_string(),
                apellido: "Pérez".to_string(),
                email: "luis@test.com".to_string(),
                dni: "456".to_string(),
                validado: true,
                balances_cripto: HashMap::new(),
                balance_fiat: 50.0,
            })
            .unwrap();
        plataforma.cotizaciones.insert("BTC".to_string(), 100.0);

        let error_fondos = plataforma.comprar_cripto(0, "BTC", 1000.0).unwrap_err();
        assert!(matches!(error_fondos, PlataformaCriptoError::FondosInsuficientes(_)));

        let error_cripto = plataforma.vender_cripto(0, "ETH", 1.0).unwrap_err();
        assert!(matches!(error_cripto, PlataformaCriptoError::CriptoNoDisponible(_)));
    }
}
