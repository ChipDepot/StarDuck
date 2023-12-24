use anyhow::Result;

pub trait UpdateState<T> {
    fn update_state(&mut self, t: T) -> Result<()>;
}
