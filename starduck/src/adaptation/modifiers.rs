use std::fmt::Display;
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ApdatationMechanism {
    Addition,
    Reconfigure,
    Alert(String),
}

impl ApdatationMechanism {
    pub fn build_default_alert(location_key: &str, data_req_key: &str, device_uuid: Option<Uuid>) -> ApdatationMechanism {
        let mut message = format!("Could not modify architecture for {data_req_key} at {location_key}.");
        
        if let Some(dev_id) = device_uuid {
            message.push_str(format!(" Related device {dev_id}").as_str());
        }

        Self::Alert(message)
    }
}

impl FromStr for ApdatationMechanism {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "addition" => Ok(Self::Addition),
            "reconfigure" | "reconfig" => Ok(Self::Reconfigure),
            _ => bail!("Invalid string {s} when converting to ApdatationMechanism"),
        }
    }
}

impl Display for ApdatationMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addition => write!(f, "addition"),
            Self::Reconfigure => write!(f, "reconfigure"),
            Self::Alert(msg) => write!(f, "Alert: {msg}"),
        }
    }
}
