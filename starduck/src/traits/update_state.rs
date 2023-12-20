use anyhow::Result;

pub trait UpdateState<T, K> {
    fn update_state(&mut self, message: T) -> Result<K>;
}
