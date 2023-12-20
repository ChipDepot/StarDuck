use std::{collections::HashMap, net::IpAddr};

use anyhow::{bail, Error, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::traits::UpdateState;
use crate::{Component, IoTOutput, SCMessage, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub status: Status,
    pub locations: HashMap<String, Location>,
    pub components: HashMap<String, Component>,
    pub properties: HashMap<String, String>,
}

impl Location {
    pub const NAME: &str = "name";
    pub const IP: &str = "ip";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";

    pub fn new(name: &str, ip: Option<IpAddr>) -> Location {
        return Location {
            name: name.to_string(),
            ip,
            status: Status::Uninitialized,
            locations: HashMap::new(),
            components: HashMap::new(),
            properties: HashMap::new(),
        };
    }

    pub fn get(&self, key: &str) -> Option<&Location> {
        if let Some(child) = self.locations.get(key) {
            return Some(child);
        }

        for (_, child) in &self.locations {
            if let Some(loc) = child.get(key) {
                return Some(loc);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Location> {
        if self.locations.contains_key(key) {
            return self.locations.get_mut(key);
        }

        for (_, child) in &mut self.locations {
            if let Some(loc) = child.get_mut(key) {
                // println!("{:?}", loc.name);
                return Some(loc);
            }
        }

        None
    }

    pub fn register_new_component(
        &mut self,
        uuid: Uuid,
        key: &str,
        iot_output: IoTOutput,
    ) -> Result<()> {
        match iot_output {
            IoTOutput::Invalid => bail!("Invalid IoTOutput recieved"),
            _ => {
                let new_comp = Component::new(key, Some(uuid), false, iot_output);
                self.components.insert(key.to_string(), new_comp);

                Ok(())
            }
        }
    }
}

impl UpdateState<&SCMessage, ()> for Location {
    fn update_state(&mut self, message: &SCMessage) -> Result<()> {
        for (key, value) in message.get_device_outputs() {
            let rec_iot = IoTOutput::from(value);

            if let Some(comp) = self.components.get_mut(&key) {
                comp.update_state(rec_iot)?;
                continue;
            };

            info!("New undeclared device output from {}", self.name);

            match self.register_new_component(message.device_uuid, &key, rec_iot) {
                Ok(_) => info!("Added {} to {}", &key, self.name),
                Err(e) => warn!("{}", e),
            }
        }

        Ok(())
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!(
            "name: {}\nip: {:?}\nlocation keys: {:?}",
            self.name,
            self.ip,
            self.locations.keys()
        )
    }
}
