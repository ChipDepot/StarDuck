use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::QueryType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestartOrder {
    pub uuid: Option<Uuid>,
    pub query_type: QueryType,
}

impl RestartOrder {
    pub fn new(query_type: QueryType) -> Self {
        Self {
            uuid: None,
            query_type,
        }
    }
}
