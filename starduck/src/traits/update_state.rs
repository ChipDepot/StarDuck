use anyhow::Result;

use crate::{CallbackMessage, SCMessage};

pub trait UpdateState {
    fn update_state(&mut self, message: &SCMessage) -> Result<CallbackMessage>;
}
