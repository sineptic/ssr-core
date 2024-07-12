use serde::{de::DeserializeOwned, Serialize};
use std::time::SystemTime;

pub trait TaskLevel: Default + Serialize + DeserializeOwned {
    type Context;
    fn update(&mut self, context: Self::Context);
    fn next_repetition(&self) -> SystemTime;
}
