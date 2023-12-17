use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::IpAddr};

use crate::components::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub locations: HashMap<String, Location>,
    pub required_components: HashMap<String, Component>,
    pub optional_components: HashMap<String, Component>,
    pub properties: HashMap<String, String>,
}

impl Location {
    pub const NAME: &str = "name";
    pub const IP: &str = "ip";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";

    pub fn new(name: &str, ip: Option<IpAddr>) -> Location {
        return Location {
            locations: HashMap::new(),
            name: name.to_string(),
            ip,
            required_components: HashMap::new(),
            optional_components: HashMap::new(),
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
