use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use serde_with::chrono::NaiveDateTime;
use uuid::Uuid;

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp_str: &str = Deserialize::deserialize(deserializer)?;

    NaiveDateTime::parse_from_str(timestamp_str, "%d-%m-%Y %H:%M:%S")
        .map_err(serde::de::Error::custom)
}

fn serialize_timestamp<S>(timestamp: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp_str = timestamp.format("%d-%m-%Y %H:%M:%S").to_string();
    serializer.serialize_str(&timestamp_str)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCMessage {
    #[serde(rename = "deviceUUID")]
    pub device_uuid: Uuid,
    pub topic: String,
    #[serde(rename = "timeStamp")]
    #[serde(
        deserialize_with = "deserialize_timestamp",
        serialize_with = "serialize_timestamp"
    )]
    pub timestamp: NaiveDateTime,
    pub values: HashMap<String, Value>,
    pub status: String,
    pub alert: bool,
}

impl SCMessage {
    const LOCATION_KEYWORD: &'static str = "location";

    pub fn get_location_key(&self) -> Option<&str> {
        self.values
            .get(Self::LOCATION_KEYWORD)
            .map(|s| s.as_str())?
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
