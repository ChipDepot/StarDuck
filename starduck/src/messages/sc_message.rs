use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCMessage {
    #[serde(rename = "deviceUUID")]
    pub device_uuid: Uuid,
    pub topic: String,
    pub timestamp: SystemTime,
    pub values: HashMap<String, Value>,
    pub status: String,
    pub alert: bool,
}

impl SCMessage {
    pub fn get_location(&self) -> Option<String> {
        self.values.get("location").map(|s| s.to_string())
    }

    pub fn get_device_outputs(&self) -> HashMap<String, Value> {
        let mut cloned_values = self.values.clone();
        cloned_values.remove("location");

        cloned_values
    }
}
