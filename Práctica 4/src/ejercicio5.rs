use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Criptomoneda {
    pub nombre: String,
    pub prefijo: String,
    pub blockchains: Vec<Blockchain>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub dni: String,
    pub validado: bool,
    pub balances_cripto: HashMap<String, f64>,
    pub balance_fiat: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaccion {
    pub fecha: String,
    pub usuario: String,
    pub tipo: String,
    pub monto: f64,
    pub cotizacion: Option<f64>,
    pub cripto: Option<String>,
    pub blockchain: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PlataformaCripto {
    pub usuarios: Vec<Usuario>,
    pub transacciones: Vec<Transaccion>,
    pub cotizaciones: HashMap<String, f64>,
}

impl PlataformaCripto {
    pub fn ingresar_fiat(&mut self, usuario_idx: usize, monto: f64) {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
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
        }
    }

    pub fn comprar_cripto(&mut self, usuario_idx: usize, cripto: &str, monto_fiat: f64) {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
            if !usuario.validado {
                return;
            }
            if let Some(cotizacion) = self.cotizaciones.get(cripto) {
                if usuario.balance_fiat >= monto_fiat {
                    let cantidad = monto_fiat / cotizacion;
                    usuario.balance_fiat -= monto_fiat;
                    *usuario.balances_cripto.entry(cripto.to_string()).or_insert(0.0) += cantidad;
                    self.transacciones.push(Transaccion {
                        fecha: "2026-01-01".to_string(),
                        usuario: usuario.nombre.clone(),
                        tipo: "compra de cripto".to_string(),
                        monto: cantidad,
                        cotizacion: Some(*cotizacion),
                        cripto: Some(cripto.to_string()),
                        blockchain: None,
                    });
                }
            }
        }
    }

    pub fn vender_cripto(&mut self, usuario_idx: usize, cripto: &str, monto_cripto: f64) {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
            if !usuario.validado {
                return;
            }
            if let Some(cotizacion) = self.cotizaciones.get(cripto) {
                let available = usuario.balances_cripto.get(cripto).copied().unwrap_or(0.0);
                if available >= monto_cripto {
                    usuario.balances_cripto.insert(cripto.to_string(), available - monto_cripto);
                    usuario.balance_fiat += monto_cripto * cotizacion;
                    self.transacciones.push(Transaccion {
                        fecha: "2026-01-01".to_string(),
                        usuario: usuario.nombre.clone(),
                        tipo: "venta de cripto".to_string(),
                        monto: monto_cripto,
                        cotizacion: Some(*cotizacion),
                        cripto: Some(cripto.to_string()),
                        blockchain: None,
                    });
                }
            }
        }
    }

    pub fn retirar_cripto(&mut self, usuario_idx: usize, cripto: &str, blockchain: &str, monto: f64) -> String {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
            if let Some(balance) = usuario.balances_cripto.get_mut(cripto) {
                if *balance >= monto {
                    *balance -= monto;
                    self.transacciones.push(Transaccion {
                        fecha: "2026-01-01".to_string(),
                        usuario: usuario.nombre.clone(),
                        tipo: "retiro cripto".to_string(),
                        monto,
                        cotizacion: None,
                        cripto: Some(cripto.to_string()),
                        blockchain: Some(blockchain.to_string()),
                    });
                    return format!("{}-{}", blockchain, self.transacciones.len());
                }
            }
        }
        "sin-hash".to_string()
    }

    pub fn recibir_cripto(&mut self, usuario_idx: usize, cripto: &str, blockchain: &str, monto: f64) {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
            *usuario.balances_cripto.entry(cripto.to_string()).or_insert(0.0) += monto;
            self.transacciones.push(Transaccion {
                fecha: "2026-01-01".to_string(),
                usuario: usuario.nombre.clone(),
                tipo: format!("recepción cripto en {}", blockchain),
                monto,
                cotizacion: None,
                cripto: Some(cripto.to_string()),
                blockchain: Some(blockchain.to_string()),
            });
        }
    }

    pub fn retirar_fiat(&mut self, usuario_idx: usize, monto: f64, medio: &str) {
        if let Some(usuario) = self.usuarios.get_mut(usuario_idx) {
            if usuario.balance_fiat >= monto {
                usuario.balance_fiat -= monto;
                self.transacciones.push(Transaccion {
                    fecha: "2026-01-01".to_string(),
                    usuario: usuario.nombre.clone(),
                    tipo: format!("retiro fiat via {}", medio),
                    monto,
                    cotizacion: None,
                    cripto: None,
                    blockchain: None,
                });
            }
        }
    }

    pub fn cripto_mas_compras(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for tx in &self.transacciones {
            if tx.tipo == "compra de cripto" {
                if let Some(cripto) = &tx.cripto {
                    *counts.entry(cripto.clone()).or_insert(0usize) += 1;
                }
            }
        }
        counts.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(cripto, _)| cripto)
    }

    pub fn cripto_mas_ventas(&self) -> Option<String> {
        let mut counts = HashMap::new();
        for tx in &self.transacciones {
            if tx.tipo == "venta de cripto" {
                if let Some(cripto) = &tx.cripto {
                    *counts.entry(cripto.clone()).or_insert(0usize) += 1;
                }
            }
        }
        counts.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(cripto, _)| cripto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plataforma_cripto() {
        let mut plataforma = PlataformaCripto::default();
        plataforma.usuarios.push(Usuario {
            nombre: "Ana".to_string(),
            apellido: "García".to_string(),
            email: "ana@test.com".to_string(),
            dni: "123".to_string(),
            validado: true,
            balances_cripto: HashMap::new(),
            balance_fiat: 1000.0,
        });
        plataforma.cotizaciones.insert("BTC".to_string(), 100.0);

        plataforma.ingresar_fiat(0, 500.0);
        plataforma.comprar_cripto(0, "BTC", 200.0);

        assert_eq!(plataforma.usuarios[0].balance_fiat, 1300.0);
        assert_eq!(plataforma.usuarios[0].balances_cripto["BTC"], 2.0);
        assert_eq!(plataforma.cripto_mas_compras(), Some("BTC".to_string()));
    }
}
