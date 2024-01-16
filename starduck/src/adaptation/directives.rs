use serde::{Deserialize, Serialize};

use super::{AdditionOrder, ReconfigureOrder, RestartOrder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directives {
    pub addition: Option<AdditionOrder>,
    pub reconfig: Option<ReconfigureOrder>,
    pub restart: Option<RestartOrder>,
}

impl Directives {
    pub fn new() -> Self {
        Directives {
            addition: None,
            reconfig: None,
            restart: None,
        }
    }
}
