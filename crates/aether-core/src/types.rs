//! Универсальные типы для гибридной инфраструктуры

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

/// Уникальный идентификатор бэкенда в гетерогенной среде
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendId {
    /// Docker контейнер: docker://<container-id>
    Docker(String),
    /// Kubernetes Pod: k8s://<namespace>/<pod-name>
    Kubernetes { namespace: String, name: String },
    /// Виртуальная машина: vm://<provider>/<instance-id>
    Vm { provider: String, instance_id: String },
    /// Голый сервер: bare://<hostname-or-ip>
    Bare(String),
}

/// Тип среды выполнения бэкенда
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendKind {
    Docker,
    Kubernetes,
    Vm,
    BareMetal,
}

/// Расширенный сокет-адрес с поддержкой доменных имен
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocketAddrExt {
    /// IP-адрес и порт
    Ip(SocketAddr),
    /// Доменное имя (для балансировки на уровне приложения)
    Domain(String, u16),
}

impl SocketAddrExt {
    pub fn port(&self) -> u16 {
        match self {
            SocketAddrExt::Ip(addr) => addr.port(),
            SocketAddrExt::Domain(_, port) => *port,
        }
    }

    pub fn ip_or_resolve(&self) -> Option<IpAddr> {
        match self {
            SocketAddrExt::Ip(addr) => Some(addr.ip()),
            SocketAddrExt::Domain(domain, _) => {
                // Разрешение домена будет происходить в рантайме
                // Здесь возвращаем None — разрешение отложено
                None
            }
        }
    }
}

/// Метки для классификации бэкендов
pub type Labels = std::collections::HashMap<String, String>;

/// Зона доступности (регион/зона/стоечный ряд)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Locality {
    pub region: String,
    pub zone: Option<String>,
    pub rack: Option<String>,
}