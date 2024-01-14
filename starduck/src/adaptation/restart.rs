use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Http { endpoint: PathBuf },
}

impl QueryType {
    pub fn get_endpoint(&self) -> Option<PathBuf> {
        match self {
            QueryType::Http { endpoint } => Some(endpoint.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartOrder {
    pub uuid: Uuid,
    pub query_type: QueryType,
}

impl RestartOrder {
    pub fn new(uuid: Uuid, query_type: QueryType) -> Self {
        Self { uuid, query_type }
    }
}
