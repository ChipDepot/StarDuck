use std::str::FromStr;

use anyhow::{bail, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Uninitialized,
    Coherent,
    Degraded,
    Critical,
    Fault,
}

impl ToString for ApplicationStatus {
    fn to_string(&self) -> String {
        match self {
            ApplicationStatus::Uninitialized => String::from("uninitialized"),
            ApplicationStatus::Coherent => String::from("coherent"),
            ApplicationStatus::Degraded => String::from("degraded"),
            ApplicationStatus::Critical => String::from("critical"),
            ApplicationStatus::Fault => String::from("fault"),
        }
    }
}

impl FromStr for ApplicationStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "uninitialized" => Ok(Self::Uninitialized),
            "coherent" => Ok(Self::Coherent),
            "degraded" => Ok(Self::Degraded),
            "critical" => Ok(Self::Critical),
            "fault" => Ok(Self::Fault),
            _ => bail!("{} is not a valid ApplicationStatus string", s),
        }
    }
}
