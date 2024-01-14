use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RestartOrder {
    pub uuid: Uuid,
}

impl RestartOrder {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}
