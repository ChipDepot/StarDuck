use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

use crate::{traits::UpdateState, Status};

use super::{Component, IoTOutput};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataRequirement {
    pub components: Vec<Component>,
    pub required_count: usize,
    #[serde(default)]
    pub timeout: Option<Duration>,
    pub status: Status,
    pub output: IoTOutput,
}

impl DataRequirement {
    pub fn new(required_count: usize, timeout: Option<Duration>, output: IoTOutput) -> Self {
        Self {
            components: Vec::new(),
            required_count,
            timeout,
            status: Status::Uninitialized,
            output,
        }
    }

    pub fn with_defaults(component: Component, output: IoTOutput) -> Self {
        let default_count = 0;
        let default_duration = None;

        let mut new_data_req = Self::new(default_count, default_duration, output);
        new_data_req.status = Status::Coherent;
        new_data_req.components.push(component);

        new_data_req
    }
}

impl UpdateState<IoTOutput, ()> for DataRequirement {
    fn update_state(&mut self, iot_output: IoTOutput) -> Result<()> {
        todo!()
    }
}

impl UpdateState<SystemTime, ()> for DataRequirement {
    fn update_state(&mut self, timestamp: SystemTime) -> Result<()> {
        todo!()
    }
}

impl Display for DataRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
