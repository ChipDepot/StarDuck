use std::fmt::Display;
use std::str::FromStr;

use anyhow::{bail, Error, Result};

enum ApdatatioMechanism {
    Relauch,
    Mirror,
    Message,
}

impl FromStr for ApdatatioMechanism {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "relauch" => Ok(Self::Relauch),
            "mirror" => Ok(Self::Mirror),
            "message" => Ok(Self::Message),
            _ => bail!("Invalid string {s} when converting to ApdatatioMechanism"),
        }
    }
}

impl Display for ApdatatioMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApdatatioMechanism::Relauch => write!(f, "relauch"),
            ApdatatioMechanism::Mirror => write!(f, "mirror"),
            ApdatatioMechanism::Message => write!(f, "message"),
        }
    }
}
