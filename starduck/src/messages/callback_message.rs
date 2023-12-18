use anyhow::Error;

use crate::Status;

pub struct CallbackMessage {
    pub required_component_status: Vec<Status>,
    pub optional_component_status: Vec<Status>,
    pub errors: Vec<Error>,
}

impl CallbackMessage {
    pub fn new() -> Self {
        CallbackMessage {
            required_component_status: Vec::new(),
            optional_component_status: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn push_required(&mut self, status: Status) {
        self.required_component_status.push(status);
    }

    pub fn push_optional(&mut self, status: Status) {
        self.optional_component_status.push(status);
    }

    pub fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
