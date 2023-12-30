use std::{fmt::Display, ops::Add};

use anyhow::Result;
use chrono::{Duration, NaiveDateTime};
use log::debug;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use serde_with::serde_as;

use crate::{
    traits::{UpdateState, UpdateStateFrom},
    SCMessage, Status,
};

use super::{Component, IoTOutput};

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataRequirement {
    pub components: Vec<Component>,
    pub required_count: usize,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub timeout: Option<Duration>,
    pub status: Status,
    pub output: IoTOutput,
}

impl DataRequirement {
    pub fn new(required_count: usize, timeout: Option<Duration>, output: IoTOutput) -> Self {
        if let Some(dur) = timeout {
            if dur.is_zero() {}
        }

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

    pub fn get_component_by_uuid(&self, uuid_to_find: Uuid) -> Option<&Component> {
        self.components
            .iter()
            .find(|&component| component.uuid.map_or(false, |uuid| uuid == uuid_to_find))
    }

    pub fn find_mut_component_by_uuid(&mut self, uuid_to_find: Uuid) -> Option<&mut Component> {
        self.components
            .iter_mut()
            .find(|component| component.uuid.map_or(false, |uuid| uuid == uuid_to_find))
    }

    fn valid_component_count(&self) -> usize {
        self.components
            .iter()
            .filter(|&comp| comp.status == Status::Coherent)
            .count()
    }

    fn set_all_component_status(&mut self, status: Status) {
        for comp in self.components.iter_mut() {
            comp.status = status;
        }
    }

    fn validate_timeout(&mut self) -> bool {
        // False means it doesn't have a timeout, true means it has a valid one
        if let Some(timeout) = self.timeout {
            if timeout.is_zero() {
                // This means that it's not really needed
                self.timeout = None;
                return false;
            }
        } else {
            return false;
        }
        true
    }
}

impl UpdateState for DataRequirement {
    fn update_state(&mut self) -> Result<()> {
        let valid_count = self.valid_component_count();

        if valid_count >= self.required_count {
            self.status = Status::Coherent;
        } else {
            self.status = Status::Fault;
        };

        return Ok(());
    }
}

impl UpdateStateFrom<&SCMessage> for DataRequirement {
    fn update_state_from(&mut self, message: &SCMessage) -> Result<()> {
        if let Some(comp) = self.find_mut_component_by_uuid(message.device_uuid) {
            comp.update_state_from(message)?;
        }

        let mut new_comp =
            Component::with_defaults(&message.generate_name(), Some(message.device_uuid));
        new_comp.update_state_from(message)?;

        Ok(())
    }
}

impl UpdateStateFrom<NaiveDateTime> for DataRequirement {
    fn update_state_from(&mut self, timestamp: NaiveDateTime) -> Result<()> {
        // Guard for timeout-less data requirements
        if !self.validate_timeout() {
            self.set_all_component_status(Status::Coherent);
            return self.update_state();
        };

        for comp in self.components.iter_mut() {
            if let Some(last_time) = comp.last_reading {
                let timeout_time = last_time.add(self.timeout.unwrap());

                let time_diff = timestamp.signed_duration_since(timeout_time);

                // Means that time timeout is earlier than the current, ergo Fault
                if time_diff.num_seconds() >= 0 {
                    comp.status = Status::Fault;
                } else {
                    comp.status = Status::Coherent;
                }

                comp.last_reading = Some(timestamp);
                continue;
            }

            // If the component has no last_reading, then we default to fault
            comp.status = Status::Fault;
        }

        self.update_state()
    }
}

impl Display for DataRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
