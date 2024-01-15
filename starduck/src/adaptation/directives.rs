use super::{AdditionOrder, ReconfigureOrder, RestartOrder};

pub struct Directives {
    pub addition: AdditionOrder,
    pub reconfig: ReconfigureOrder,
    pub restart: RestartOrder,
}

impl Directives {
    fn new(addition: AdditionOrder, reconfig: ReconfigureOrder, restart: RestartOrder) -> Self {
        Directives {
            addition,
            reconfig,
            restart,
        }
    }
}
