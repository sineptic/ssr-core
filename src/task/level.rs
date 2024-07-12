use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

pub trait TaskLevel: Default + Serialize + DeserializeOwned {
    type Context;
    fn update(&mut self, context: Self::Context);
    fn until_next_repetition(&self) -> Duration;
}
