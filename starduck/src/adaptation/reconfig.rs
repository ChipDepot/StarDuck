use std::path::PathBuf;

use http::{Method, Uri};
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
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconfigureOrder {
    pub uuid: Uuid,
    pub network: String,
    pub query_type: QueryType,
    pub reconfig: ReconfigureType,
}

impl ReconfigureOrder {
    pub fn new(
        uuid: Uuid,
        network: &str,
        reconfig: ReconfigureType,
        query_type: QueryType,
    ) -> Self {
        Self {
            uuid,
            network: network.to_string(),
            reconfig,
            query_type,
        }
    }
}
