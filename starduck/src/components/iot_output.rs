use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Status;

use super::ComponentError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum IoTOutput {
    Number(Status),
    Boolean(Status),
    Text(Status),
    Invalid,
}

impl Display for IoTOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoTOutput::Number(_) => write!(f, "Number"),
            IoTOutput::Boolean(_) => write!(f, "Boolean"),
            IoTOutput::Text(_) => write!(f, "Text"),
            IoTOutput::Invalid => write!(f, "Invalid"),
        }
    }
}

impl From<Value> for IoTOutput {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(_) => IoTOutput::Number(Status::Uninitialized),
            Value::Bool(_) => IoTOutput::Boolean(Status::Uninitialized),
            Value::String(_) => IoTOutput::Text(Status::Uninitialized),
            _ => IoTOutput::Invalid,
        }
    }
}

impl FromStr for IoTOutput {
    type Err = ComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "number" => Ok(IoTOutput::Number(Status::Uninitialized)),
            "bool" => Ok(IoTOutput::Boolean(Status::Uninitialized)),
            "text" => Ok(IoTOutput::Text(Status::Uninitialized)),
            _ => Err(ComponentError::InvalidIoTOutput(s.to_string())),
        }
    }
}
