use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{ComponentType, IoTOutput, Property};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub component_type: ComponentType,
    pub properties: HashMap<String, Property>,
    pub outputs: HashMap<String, IoTOutput>,
}

impl Component {
    pub const NAME: &str = "name";
    pub const COMPONENT_TYPE: &str = "component-type";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";
    pub const OUTPUTS: &str = "outputs";

    pub fn new(name: String, component_type: ComponentType) -> Component {
        Component {
            name,
            uuid: None,
            component_type,
            properties: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}

impl ToString for Component {
    fn to_string(&self) -> String {
        format!(
            "name: {}\nuuid: {}\ncomponent_type: {}\nproperties: {:?}\noutputs: {:?}",
            self.name,
            self.uuid
                .map_or_else(|| "None".to_string(), |k| k.to_string()),
            self.component_type.to_string(),
            self.properties,
            self.outputs
        )
        .to_string()
    }
}
