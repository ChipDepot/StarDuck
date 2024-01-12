use super::ApdatationMechanism;

#[derive(Debug, Clone)]
pub struct Doctrine {
    pub class: ApdatationMechanism,
    pub data_requirement_key: String,
    pub cmd: String,
}

impl Doctrine {
    pub fn new(class: ApdatationMechanism, data_requirement_key: String, cmd: String) -> Self {
        Self {
            class,
            data_requirement_key,
            cmd,
        }
    }
}
