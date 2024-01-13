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
        image: String,
        network_name: String,
        env_vars: HashMap<String, Value>,
        args: Vec<String>,
    ) -> Self {
        Self {
            image,
            network_name,
            env_vars,
            args,
        }
    }
}
