use std::collections::HashMap;

use super::output::IoTOutput;
use super::properties::Property;

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub properties: HashMap<String, Property>,
    pub outputs: HashMap<String, IoTOutput>,
}

impl Component {}
