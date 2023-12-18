use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{traits::UpdateState, CallbackMessage, SCMessage, Status};

use super::{ComponentType, IoTOutput};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub component_type: ComponentType,
    pub required: bool,
    pub status: Status,
    pub outputs: HashMap<String, IoTOutput>,
}

impl Component {
    pub const NAME: &str = "name";
    pub const COMPONENT_TYPE: &str = "component-type";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";
    pub const OUTPUTS: &str = "outputs";

    pub fn new(name: String, required: bool, component_type: ComponentType) -> Component {
        Component {
            name,
            uuid: None,
            component_type,
            required,
            status: Status::Uninitialized,
            outputs: HashMap::new(),
        }
    }
}

impl UpdateState for Component {
    fn update_state(&mut self, message: &SCMessage) -> Result<CallbackMessage> {
        todo!()
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nuuid: {}\ncomponent_type: {}\noutputs: {:?}",
            self.name,
            self.uuid
                .map_or_else(|| "None".to_string(), |k| k.to_string()),
            self.component_type.to_string(),
            self.outputs
        )
    }
}
