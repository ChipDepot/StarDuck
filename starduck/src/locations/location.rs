use std::{collections::HashMap, net::IpAddr};

use anyhow::{bail, Error, Result};
use serde::{Deserialize, Serialize};

use crate::traits::UpdateState;
use crate::{CallbackMessage, Component, IoTOutput, SCMessage, Status};

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

    pub fn get_mut_component_by_output(&mut self, output_key: &str) -> Option<&mut Component> {
        self.components
            .values_mut()
            .find(|comp| comp.outputs.contains_key(output_key))
    }
}

impl UpdateState for Location {
    fn update_state(&mut self, message: &SCMessage) -> Result<CallbackMessage> {
        let mut call_message = CallbackMessage::new();

        for (key, value) in message.get_device_outputs() {
            if let Some(component) = self.get_mut_component_by_output(&key) {
                let expected = component.outputs.get(&key).unwrap();
                let recieved = IoTOutput::from(value);

                if *expected != recieved {
                    component.status = Status::Fault;

                    if component.required {
                        call_message.push_required(Status::Fault);
                    } else {
                        call_message.push_required(Status::Fault);
                    }

                    bail!(
                        "Invalid value recieved from message. Expected {}, got {}",
                        expected,
                        recieved,
                    );
                }
            } else {
                bail!(
                    "Key `{}` was not found in the component list for `{}`",
                    &key,
                    &self.name
                );
            }
        }

        todo!()
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
