use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Component `{0}` has no valid component-type")]
    InvalidComponentType(String),
    #[error("`{0}` is an invalid component output")]
    InvalidIoTOutput(String),
}
