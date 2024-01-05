use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ComponentError;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum IoTOutput {
    Number,
    Boolean,
    Text,
    Invalid,
}

impl Display for IoTOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoTOutput::Number => write!(f, "number"),
            IoTOutput::Boolean => write!(f, "boolean"),
            IoTOutput::Text => write!(f, "text"),
            IoTOutput::Invalid => write!(f, "invalid"),
        }
    }
}

impl From<Value> for IoTOutput {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(_) => IoTOutput::Number,
            Value::Bool(_) => IoTOutput::Boolean,
            Value::String(_) => IoTOutput::Text,
            _ => IoTOutput::Invalid,
        }
    }
}

impl FromStr for IoTOutput {
    type Err = ComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "number" => Ok(IoTOutput::Number),
            "bool" => Ok(IoTOutput::Boolean),
            "text" => Ok(IoTOutput::Text),
            _ => Err(ComponentError::InvalidIoTOutput(s.to_string())),
        }
    }
}
