use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// All methods must be O(1)
pub trait TaskLevel: Default + Serialize + DeserializeOwned {
    fn success(&mut self);
    fn failure(&mut self);
    fn duration(&self) -> Duration;
}
