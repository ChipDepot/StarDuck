use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use anyhow::{bail, Error, Result};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Directive {
    Addition {
        image: String,
        network: String,
        env_vars: HashMap<String, Value>,
        args: Vec<String>,
    },
    Reconfigure {
        uuid: Uuid,
        network: String,
        env_vars: HashMap<String, Value>,
        args: Vec<String>,
    },
}

impl Display for Directive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
