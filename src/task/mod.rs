use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod level;

pub trait Task<'a>: Serialize + Deserialize<'a> {
    type SharedState: SharedState<'a>;

    fn next_repetition(
        &self,
        shared_state: &Self::SharedState,
        desired_retention: f64,
    ) -> SystemTime;
    /// If an error occurs, the task will remain unmodified.
    /// # Errors
    /// If interaction return error.
    fn complete(
        &mut self,
        shared_state: &mut Self::SharedState,
        desired_retention: f64,
        interaction: &mut impl FnMut(
            s_text_input_f::Blocks,
        ) -> std::io::Result<s_text_input_f::Response>,
    ) -> std::io::Result<()>;
}

pub trait SharedState<'a>: Default + Serialize + Deserialize<'a> {}
impl SharedState<'_> for () {}

pub trait SharedStateExt<'a>: SharedState<'a> {
    fn optimize(&mut self);
}
