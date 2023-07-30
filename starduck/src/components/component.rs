use std::collections::HashMap;
use thiserror::Error;

use super::properties::Property;

#[derive(Debug, Clone)]
pub enum ComponentType {
    Sensor,
    Actuator,
}

impl ComponentType {
    pub fn from_string(s: &str) -> Result<ComponentType, ComponentError> {
        match s.to_lowercase().as_str() {
            "sensor" => Ok(ComponentType::Sensor),
            "actuator" => Ok(ComponentType::Actuator),
            s => Err(ComponentError::InvalidComponentType(s.to_string())),
        }
    }
}

impl ToString for ComponentType {
    fn to_string(&self) -> String {
        match self {
            ComponentType::Sensor => String::from("sensor"),
            ComponentType::Actuator => String::from("actuator"),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Component `{0}` has no valid component-type")]
    InvalidComponentType(String),
    #[error("`{0}` is an invalid component output")]
    InvalidIoTOutput(String),
}

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub component_type: ComponentType,
    pub components: HashMap<String, Box<Component>>,
    pub properties: HashMap<String, Property>,
    pub outputs: HashMap<String, IoTOutput>,
}

impl Component {
    pub const NAME: &str = "name";
    pub const COMPONENT_TYPE: &str = "component-type";
    pub const COMPONENTS: &str = "components";
    pub const PROPERTIES: &str = "properties";
    pub const OUTPUTS: &str = "outputs";

    pub fn new(name: String, component_type: ComponentType) -> Component {
        Component {
            name,
            component_type,
            components: HashMap::new(),
            properties: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}

impl ToString for Component {
    fn to_string(&self) -> String {
        format!(
            "name: {}\ncomponent_type: {}\ncomponents: {:?}\nproperties: {:?}\noutputs: {:?}",
            self.name,
            self.component_type.to_string(),
            self.components,
            self.properties,
            self.outputs
        )
        .to_string()
    }
}
