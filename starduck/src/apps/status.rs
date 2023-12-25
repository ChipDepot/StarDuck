use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Status {
    Uninitialized,
    Coherent,
    Degraded,
    Critical,
    Fault,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Uninitialized => write!(f, "uninitialized"),
            Status::Coherent => write!(f, "coherent"),
            Status::Degraded => write!(f, "degraded"),
            Status::Critical => write!(f, "critical"),
            Status::Fault => write!(f, "fault"),
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "uninitialized" => Ok(Self::Uninitialized),
            "coherent" => Ok(Self::Coherent),
            "degraded" => Ok(Self::Degraded),
            "critical" => Ok(Self::Critical),
            "fault" => Ok(Self::Fault),
            _ => bail!("{} is not a valid Status string", s),
        }
    }
}
