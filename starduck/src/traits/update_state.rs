use anyhow::Result;
pub trait UpdateState {
    fn update_state(&mut self) -> Result<()>;
}

pub trait UpdateStateFrom<T> {
    fn update_state_from(&mut self, t: T) -> Result<()>;
}
