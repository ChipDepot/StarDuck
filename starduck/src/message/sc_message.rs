use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use uuid::Uuid;

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp_str = String::deserialize(deserializer)?;
    let format = "%d-%m-%Y %H:%M:%S";
    let naive_datetime = NaiveDateTime::parse_from_str(&timestamp_str, format);

    match naive_datetime {
        Ok(naive_dt) => {
            // Convert naive datetime to DateTime in the local timezone
            Ok(Local.from_local_datetime(&naive_dt).single().unwrap())
        }
        Err(err) => Err(serde::de::Error::custom(err)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SCMessage {
    #[serde(rename = "deviceUUID")]
    pub device_uuid: Uuid,
    pub topic: String,
    #[serde(rename = "timeStamp", deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Local>,
    pub values: HashMap<String, Value>,
    pub status: String,
    pub alert: bool,
}
