use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    Http { endpoint: PathBuf },
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
