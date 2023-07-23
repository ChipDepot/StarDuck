use std::time::Instant;

#[derive(Debug, Clone)]
pub enum IoTOutput {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Text(String),
    Timestamp(Instant),
}
