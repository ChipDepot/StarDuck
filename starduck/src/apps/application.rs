use crate::component::Component;
use crate::location::Location;

pub struct Application {
    pub name: String,
    pub locations: Location,
    pub components: Component
}

impl Application {
    pub const NAME: &str = "name";
    pub const COMPONENTS: &str = "components";
    pub const LOCATIONS: &str = "locations";

    pub fn new(name: String, locations: Location, components: Component) -> Application {
        Application {
            name,
            locations,
            components
        }
    }
}

impl ToString for Application {
    fn to_string(&self) -> String {
        format!(
            "name: {}\nLocations: {}\nComponents: {}",
            self.name,
            self.components.to_string(),
            self.locations.to_string(),
        )
    }
}