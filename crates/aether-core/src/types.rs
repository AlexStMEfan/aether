use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackendId {
    Docker(String),
    Kubernetes { namespace: String, name: String },
    Vm { provider: String, instance_id: String },
    Bare(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendKind {
    Docker,
    Kubernetes,
    Vm,
    BareMetal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocketAddrExt {
    Ip(SocketAddr),
    Domain(String, u16),
}
