use std::fmt::Display;
use std::time::SystemTime;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{traits::UpdateStateFrom, SCMessage, Status};

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

    pub fn update_timeout_status(&mut self, max_time: SystemTime) {
        if let Some(last) = self.last_reading {
            match max_time.duration_since(last) {
                Ok(_q) => todo!(),
                Err(_) => todo!(),
            }
        }
    }
}

impl UpdateStateFrom<&SCMessage> for Component {
    fn update_state_from(&mut self, message: &SCMessage) -> Result<()> {
        self.last_reading = Some(message.timestamp);
        self.status = Status::Coherent;

        Ok(())
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
