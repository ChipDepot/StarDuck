use std::time::SystemTime;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{traits::UpdateState, Location, SCMessage, Status};

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

impl UpdateState<&SCMessage> for Application {
    fn update_state(&mut self, message: &SCMessage) -> Result<()> {
        let location_key = match message.get_location_key() {
            Some(k) => k,
            None => {
                bail!("Missing `location` in message from {}", message.device_uuid);
            }
        };

        let location = match self.locations.get_mut(&location_key) {
            Some(k) => k,
            None => bail!("No location was found for key `{}`", &location_key),
        };

        self.last_update = Some(SystemTime::now());

        location.update_state(message)
    }
}

impl UpdateState<SystemTime> for Application {
    fn update_state(&mut self, timestamp: SystemTime) -> Result<()> {
        self.last_update = Some(timestamp);

        // Update child locations from root
        self.locations.update_state(timestamp)
    }
}
