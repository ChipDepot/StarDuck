use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Http { endpoint: PathBuf, port: u16 },
}

impl QueryType {
    pub fn get_endpoint(&self) -> Option<PathBuf> {
        match self {
            QueryType::Http { endpoint, .. } => Some(endpoint.clone()),
            _ => None,
        }
    }
}
