use uuid::Uuid;

use crate::adaptation::Directive;

pub struct ExecMessage {
    pub action: Directive,
    pub location_key: String,
    pub data_requirement_key: String,
    pub device_uuid: Option<Uuid>,
}

impl ExecMessage {
    pub fn new(
        action: Directive,
        location_key: String,
        data_requirement_key: String,
        device_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            action,
            location_key,
            data_requirement_key,
            device_uuid,
        }
    }
}
