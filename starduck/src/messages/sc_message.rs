use std::time::SystemTime;
use std::{collections::HashMap, fmt::Display};

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
    const LOCATION_KEYWORD: &str = "location";

    pub fn get_location_key(&self) -> Option<String> {
        self.values
            .get(Self::LOCATION_KEYWORD)
            .map(|s| s.to_string())
    }

    pub fn get_device_outputs(&self) -> HashMap<String, Value> {
        let mut cloned_values = self.values.clone();
        cloned_values.remove(Self::LOCATION_KEYWORD);

        cloned_values
    }

    pub fn generate_name(&self) -> String {
        format!("Unregistered device {}", self.device_uuid.as_simple())
    }
}

impl Display for SCMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
