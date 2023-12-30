use anyhow::{bail, Result};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    traits::{UpdateState, UpdateStateFrom},
    Location, SCMessage, Status,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub locations: Location,
    pub last_update: Option<NaiveDateTime>,
}

impl Application {
    pub const LOCATIONS: &str = "locations";

    pub fn new(name: &str, description: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|k| k.to_string()),
            status: Status::Uninitialized,
            locations: Location::new("root", None),
            last_update: None,
        }
    }
}

impl UpdateState for Application {
    fn update_state(&mut self) -> Result<()> {
        // The status of the application is the same as the status of the root
        self.status = self.locations.status;

        Ok(())
    }
}

impl UpdateStateFrom<&SCMessage> for Application {
    fn update_state_from(&mut self, message: &SCMessage) -> Result<()> {
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

        self.last_update = Some(Utc::now().naive_local());

        location.update_state_from(message)
    }
}

impl UpdateStateFrom<NaiveDateTime> for Application {
    fn update_state_from(&mut self, timestamp: NaiveDateTime) -> Result<()> {
        self.last_update = Some(timestamp);

        // Update child locations from root
        self.locations.update_state_from(timestamp)?;

        self.update_state()
    }
}
