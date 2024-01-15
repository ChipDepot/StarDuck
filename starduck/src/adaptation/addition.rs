use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
}
