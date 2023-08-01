use std::{boxed::Box, collections::HashMap, net::IpAddr};

use thiserror::Error;

use crate::components::component::Component;

#[derive(Debug, Clone)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub locations: HashMap<String, Location>,
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
