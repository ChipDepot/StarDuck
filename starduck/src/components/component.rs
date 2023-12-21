use std::fmt::Display;
use std::time::SystemTime;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{traits::UpdateState, Status};

use super::IoTOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub status: Status,
    pub last_reading: Option<SystemTime>,
}

impl Component {
    pub fn new(name: &str, uuid: Option<Uuid>) -> Component {
        Self {
            name: name.to_string(),
            uuid,
            status: Status::Uninitialized,
            last_reading: None,
        }
    }

    pub fn with_defaults(name: &str, uuid: Option<Uuid>) -> Self {
        let mut new_comp = Self::new(name, uuid);
        new_comp.status = Status::Coherent;
        new_comp.last_reading = Some(SystemTime::now());

        new_comp
    }
}

impl UpdateState<IoTOutput, ()> for Component {
    fn update_state(&mut self, message: IoTOutput) -> Result<()> {
        todo!()
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
