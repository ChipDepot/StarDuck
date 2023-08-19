use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::component::Component;
use crate::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    // pub name: String,
    pub locations: HashMap<String, Box<Location>>,
    pub components: HashMap<String, Component>
}

impl Application {
    pub const NAME: &str = "name";
    pub const COMPONENTS: &str = "components";
    pub const LOCATIONS: &str = "locations";

    pub fn new(locations: HashMap<String, Box<Location>>, components: HashMap<String, Component>) -> Application {
        Application {
            locations,
            components
        }
    }
}
