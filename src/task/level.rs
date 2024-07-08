use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub trait TaskLevel<C>: Default + Serialize + DeserializeOwned {
    fn success(&mut self, context: C);
    fn failure(&mut self, context: C);
    fn duration(&self) -> Duration;
}
