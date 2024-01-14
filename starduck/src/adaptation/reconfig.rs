use std::path::PathBuf;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReconfigureType {
    Http {
        endpoint: PathBuf,
        port: u16,
        #[serde(with = "http_serde::method")]
        method: Method,
        payload: Value,
    },
}

impl ReconfigureType {
    pub fn get_endpoint(&self) -> Option<PathBuf> {
        match self {
            ReconfigureType::Http { endpoint, .. } => Some(endpoint.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigureOrder {
    pub uuid: Uuid,
    pub network: String,
    pub reconfig: ReconfigureType,
}

impl ReconfigureOrder {
    pub fn new(uuid: Uuid, network: &str, reconfig: ReconfigureType) -> Self {
        Self {
            uuid,
            network: network.to_string(),
            reconfig,
        }
    }
}
