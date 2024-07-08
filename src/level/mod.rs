use serde::{de::DeserializeOwned, Serialize};
use std::time::{Duration, SystemTime};

/// All methods must be O(1)
pub trait TaskLevel: Default + Serialize + DeserializeOwned {
    fn success(&mut self, current_time: SystemTime);
    fn failure(&mut self, current_time: SystemTime);
    fn duration(&self) -> Duration;
}
