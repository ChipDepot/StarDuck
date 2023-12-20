use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{traits::UpdateState, Status};

use super::IoTOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub required: bool,
    pub status: Status,
    pub output: IoTOutput,
}

impl Component {
    pub const NAME: &str = "name";
    pub const COMPONENT_TYPE: &str = "component-type";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";
    pub const OUTPUTS: &str = "outputs";

    pub fn new(name: &str, uuid: Option<Uuid>, required: bool, output: IoTOutput) -> Component {
        Component {
            name: name.to_string(),
            uuid,
            required,
            status: Status::Uninitialized,
            output,
        }
    }
}

impl UpdateState<IoTOutput, ()> for Component {
    fn update_state(&mut self, iot_output: IoTOutput) -> Result<()> {
        if self.output == iot_output {
            self.status = Status::Coherent;
        } else {
            self.status = Status::Fault;
        }

        Ok(())
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\nuuid: {}\noutput: {}",
            self.name,
            self.uuid
                .map_or_else(|| "None".to_string(), |k| k.to_string()),
            self.output
        )
    }
}
