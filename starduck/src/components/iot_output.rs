use serde::{Deserialize, Serialize};

use super::ComponentError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IoTOutput {
    Integer,
    Float,
    Boolean,
    Text,
    Timestamp,
}

impl IoTOutput {
    pub fn from_string(s: String) -> Result<IoTOutput, ComponentError> {
        match s.to_lowercase().as_str() {
            "int" => Ok(IoTOutput::Integer),
            "integer" => Ok(IoTOutput::Integer),
            "float" => Ok(IoTOutput::Float),
            "bool" => Ok(IoTOutput::Boolean),
            "text" => Ok(IoTOutput::Text),
            "time" => Ok(IoTOutput::Timestamp),
            _ => Err(ComponentError::InvalidIoTOutput(s)),
        }
    }
}
