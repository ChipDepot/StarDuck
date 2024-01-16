use std::collections::HashMap;

use anyhow::{bail, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdditionOrder {
    pub image: String,
    pub network_name: String,
    pub env_vars: HashMap<String, Value>,
    pub args: Vec<String>,
}

impl AdditionOrder {
    pub fn new(
        image: &str,
        network_name: &str,
        env_vars: HashMap<String, Value>,
        args: Vec<String>,
    ) -> Self {
        Self {
            image: image.to_string(),
            network_name: network_name.to_string(),
            env_vars,
            args,
        }
    }

    pub fn get_uuid(&self) -> Result<Uuid> {
        let matching_uuids: Vec<Uuid> = self
            .args
            .iter()
            .filter_map(|arg| {
                if let Some(uuid_str) = arg.strip_prefix("device_uuid=") {
                    return Uuid::parse_str(uuid_str).ok();
                }
                None
            })
            .collect();

        match matching_uuids.len() {
            0 => bail!("No matching device_uuid found"),
            1 => Ok(matching_uuids[0]),
            _ => bail!("More than one matching device_uuid found"),
        }
    }
}
