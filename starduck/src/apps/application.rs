use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::location::Location;
use crate::sc_message::SCMessage;

use super::ApplicationStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub status: ApplicationStatus,
    pub locations: Location,
    pub last_update: Option<DateTime<Local>>,
}

impl Application {
    pub const LOCATIONS: &str = "locations";

    pub fn new(name: &str) -> Application {
        Application {
            name: name.to_string(),
            status: ApplicationStatus::Uninitialized,
            locations: Location::new("root", None),
            last_update: None,
        }
    }

    pub fn update_state(&mut self, message: &SCMessage) -> Result<()> {
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

        Ok(())
    }
}
