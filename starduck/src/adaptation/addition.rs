use std::collections::HashMap;

use anyhow::{bail, Context, Result};

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
        if let Some(uuid) = self.env_vars.get("device_uuid").map(|arg| {
            return Uuid::parse_str(arg.as_str().unwrap());
        }) {
            return uuid.with_context(|| "Could not parse Str as Uuid");
        }

        bail!("No matching device_uuid found");
    }
}
