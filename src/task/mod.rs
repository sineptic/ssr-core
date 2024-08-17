use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod level;

pub trait Task<'a>: Serialize + Deserialize<'a> {
    type SharedState: SharedState<'a>;

    fn get_desctiption(&self) -> &str;

    fn next_repetition(&self, _retrievability_goal: f64) -> SystemTime;
    /// If an error occurs, the task will remain unmodified.
    fn complete(
        &self,
        shared_state: &mut Self::SharedState,
        interaction: &mut impl FnMut(
            &s_text_input_f::Blocks,
        ) -> std::io::Result<s_text_input_f::Response>,
    ) -> std::io::Result<()>;
}

pub trait SharedState<'a>: Default + Serialize + Deserialize<'a> {}
impl<'a> SharedState<'a> for () {}

pub trait SharedStateExt<'a>: SharedState<'a> {
    fn optimize(&mut self);
}
