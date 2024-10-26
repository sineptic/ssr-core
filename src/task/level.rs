use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub trait TaskLevel<'a>: Serialize + Deserialize<'a> {
    type Context;
    type SharedState: Default + Serialize + Deserialize<'a>;
    fn update(&mut self, shared_state: &mut Self::SharedState, context: Self::Context);
    fn next_repetition(
        &self,
        shared_state: &Self::SharedState,
        retrievability_goal: f64,
    ) -> SystemTime;
}
