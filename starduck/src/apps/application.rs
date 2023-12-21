use std::time::SystemTime;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{traits::UpdateState, CallbackMessage, Location, SCMessage, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub status: Status,
    pub locations: Location,
    pub last_update: Option<SystemTime>,
}

impl Application {
    pub const LOCATIONS: &str = "locations";

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: Status::Uninitialized,
            locations: Location::new("root", None),
            last_update: None,
        }
    }
}

impl UpdateState<&SCMessage, CallbackMessage> for Application {
    fn update_state(&mut self, message: &SCMessage) -> Result<CallbackMessage> {
        let location_key = match message.get_location() {
            Some(k) => k,
            None => {
                bail!("Missing `location` in message from {}", message.device_uuid);
            }
        };

        let location = match self.locations.get_mut(&location_key) {
            Some(k) => k,
            None => bail!("No location was found for key `{}`", &location_key),
        };

        match location.update_state(message) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };
    }
}
