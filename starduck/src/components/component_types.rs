use serde::{Deserialize, Serialize};

use super::ComponentError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ComponentType {
    Sensor,
    Actuator,
    Aggregator,
    Processor,
}

impl ComponentType {
    pub fn from_string(s: &str) -> Result<ComponentType, ComponentError> {
        match s.to_lowercase().as_str() {
            "sensor" => Ok(ComponentType::Sensor),
            "actuator" => Ok(ComponentType::Actuator),
            "aggregator" => Ok(ComponentType::Aggregator),
            "processor" => Ok(ComponentType::Processor),
            s => Err(ComponentError::InvalidComponentType(s.to_string())),
        }
    }
}

impl ToString for ComponentType {
    fn to_string(&self) -> String {
        match self {
            ComponentType::Sensor => String::from("sensor"),
            ComponentType::Actuator => String::from("actuator"),
            ComponentType::Aggregator => String::from("aggregator"),
            ComponentType::Processor => String::from("processor"),
        }
    }
}
