use std::{collections::HashMap, net::IpAddr};

use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::traits::{UpdateState, UpdateStateFrom};
use crate::{Component, DataRequirement, IoTOutput, SCMessage, Status};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub status: Status,
    pub locations: HashMap<String, Location>,
    pub data_requirements: HashMap<String, DataRequirement>,
    pub properties: HashMap<String, String>,
}

impl Location {
    pub fn new(name: &str, ip: Option<IpAddr>) -> Self {
        Self {
            name: name.to_string(),
            ip,
            status: Status::Uninitialized,
            locations: HashMap::new(),
            data_requirements: HashMap::new(),
            properties: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Self> {
        if let Some(child) = self.locations.get(key) {
            return Some(child);
        }

        for (_, child) in &self.locations {
            if let Some(loc) = child.get(key) {
                return Some(loc);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Self> {
        if self.locations.contains_key(key) {
            return self.locations.get_mut(key);
        }

        for (_, child) in &mut self.locations {
            if let Some(loc) = child.get_mut(key) {
                // println!("{:?}", loc.name);
                return Some(loc);
            }
        }

        None
    }

    pub fn register_new_component(
        &mut self,
        uuid: Uuid,
        key: &str,
        iot_output: IoTOutput,
    ) -> Result<()> {
        match iot_output {
            IoTOutput::Invalid => bail!("Invalid IoTOutput recieved"),
            _ => {
                let new_comp = Component::with_defaults(key, Some(uuid));

                if let Some(data_req) = self.data_requirements.get_mut(key) {
                    data_req.components.push(new_comp);
                    return Ok(());
                }

                // let new_data_req = DataRequirement::with_defaults(new_comp, iot_output);
                // self.data_requirements.insert(key.to_string(), new_data_req);

                Ok(())
            }
        }
    }

    fn get_data_requirement_statuses_count(&self, required: bool, status: Status) -> usize {
        self.data_requirements
            .values()
            .filter(|k| k.required == required && k.status == status)
            .count()
    }

    fn get_data_requirement_count(&self, required: bool) -> usize {
        self.data_requirements
            .iter()
            .filter(|(_, data)| data.required == required)
            .count()
    }

    pub fn get_locations_status(&self) -> Vec<Status> {
        self.locations
            .values()
            .map(|data_req| data_req.status.clone())
            .collect()
    }
}

impl UpdateState for Location {
    fn update_state(&mut self) -> Result<()> {
        if self.locations.is_empty() {
            // Data requirement location
            let req_data_statuses_count =
                self.get_data_requirement_statuses_count(true, Status::Coherent);
            let total_req_data_req = self.get_data_requirement_count(true);

            // Faults if any of the required components are in Fault
            if req_data_statuses_count != total_req_data_req {
                self.status = Status::Fault;
                return Ok(());
            }

            let opt_data_statuses_count =
                self.get_data_requirement_statuses_count(false, Status::Coherent);
            let total_opt_data_req = self.get_data_requirement_count(false);

            self.status = match (opt_data_statuses_count, total_opt_data_req) {
                (count, total) if count == total => Status::Coherent,
                (0, _) => Status::Critical,
                _ => Status::Degraded,
            };

            return Ok(());
        } else {
            let loc_statuses = self.get_locations_status();

            self.status = match () {
                _ if loc_statuses.contains(&Status::Fault) => Status::Fault,
                _ if loc_statuses.contains(&Status::Critical) => Status::Critical,
                _ if loc_statuses.contains(&Status::Degraded) => Status::Degraded,
                _ => Status::Coherent,
            };
        }

        Ok(())
    }
}

impl UpdateStateFrom<&SCMessage> for Location {
    fn update_state_from(&mut self, message: &SCMessage) -> Result<()> {
        for (key, _value) in message.get_device_outputs() {
            // let rec_iot = IoTOutput::from(value);

            if let Some(data_req) = self.data_requirements.get_mut(&key) {
                data_req.update_state_from(message)?;
                continue;
            };

            warn!("Undeclared device output {} from {}", &key, self.name);

            // match self.register_new_component(message.device_uuid, &key, rec_iot) {
            //     Ok(_) => info!("Added {} to {}", &key, self.nam e),
            //     Err(e) => warn!("{}", e),
            // }
        }

        Ok(())
    }
}

impl UpdateStateFrom<NaiveDateTime> for Location {
    fn update_state_from(&mut self, timestamp: NaiveDateTime) -> Result<()> {
        // Update local data requirements
        for (_, data_req) in self.data_requirements.iter_mut() {
            data_req.update_state_from(timestamp)?;
        }

        // Update child locations
        for (_, loc) in self.locations.iter_mut() {
            loc.update_state_from(timestamp)?;
        }

        self.update_state()
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!(
            "name: {}\nip: {:?}\nlocation keys: {:?}",
            self.name,
            self.ip,
            self.locations.keys()
        )
    }
}
