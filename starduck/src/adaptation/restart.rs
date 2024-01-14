use uuid::Uuid;

pub struct RestartOrder {
    pub uuid: Uuid,
}

impl RestartOrder {
    pub fn new(uuid: Uuid) -> Self {
        Self { uuid }
    }
}
