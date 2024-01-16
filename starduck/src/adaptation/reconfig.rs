use std::path::PathBuf;

use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::QueryType;

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
    pub fn build_url<I: AsRef<str>>(&self, domain: I) -> Option<String> {
        match self {
            ReconfigureType::Http { endpoint, port, .. } => Some(format!(
                "https://{}:{}{}",
                domain.as_ref(),
                port,
                endpoint.to_string_lossy()
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigureOrder {
    pub uuid: Option<Uuid>,
    pub network: String,
    pub query_type: QueryType,
    pub reconfig: ReconfigureType,
}

impl ReconfigureOrder {
    pub fn new(network: &str, reconfig: ReconfigureType, query_type: QueryType) -> Self {
        Self {
            uuid: None,
            network: network.to_string(),
            reconfig,
            query_type,
        }
    }
}
