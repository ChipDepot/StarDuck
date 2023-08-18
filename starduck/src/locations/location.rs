use std::{collections::HashMap, net::IpAddr};
use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::components::component::Component;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub locations: HashMap<String, Box<Location>>,
    pub components: HashMap<String, Component>,
    pub properties: HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum LocationError {
    #[error("Location `{0}` has no child locations or an ip")]
    NoLocationIp(String),
}

impl Location {
    pub const NAME: &str = "name";
    pub const IP: &str = "ip";
    pub const LOCATIONS: &str = "locations";
    pub const PROPERTIES: &str = "properties";

    pub fn new(name: String, ip: Option<IpAddr>) -> Location {
        return Location {
            locations: HashMap::new(),
            name,
            ip,
            components: HashMap::new(),
            properties: HashMap::new(),
        };
    }

    pub fn get(&self, key: &str) -> Option<&Box<Location>> {
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
        if let Some(location) = self.get(key) {
            // Clone the name to avoid ownership issues
            let location_name = location.name.clone();
            return self.locations.get_mut(&location_name).map(|boxed_loc| &mut **boxed_loc);
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
