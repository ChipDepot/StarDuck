use serde::{Deserialize, Serialize};

use super::{AdditionOrder, ReconfigureOrder, RestartOrder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directives {
    pub addition: AdditionOrder,
    pub reconfig: ReconfigureOrder,
    pub restart: RestartOrder,
}

impl Directives {
    pub fn new(addition: AdditionOrder, reconfig: ReconfigureOrder, restart: RestartOrder) -> Self {
        Directives {
            addition,
            reconfig,
            restart,
        }
    }
}
