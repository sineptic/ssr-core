use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub mod level;

#[derive(Debug, Clone)]
pub enum Feedback {
    CorrectAnswer,
    WrongAnswer {
        correct_answers: Vec<String>,
        explanation: Option<String>,
    },
}

pub enum InterationItem {
    /// Does not have intractivity
    Text(String),
    /// Any utf8 input from user
    BlankField,
    /// One vector element
    OneOf(Vec<(u32, String)>),
    /// Any set of vector elements
    AnyOf(Vec<(u32, String)>),
    /// Any order of all items
    Order(Vec<(u32, String)>),
}
pub trait UserInteraction {
    /// # Returns
    /// Vector of all interaction results in the same order
    /// # For implementors
    /// - Text must be displayed without modification.
    /// - `BlankField` must produce String, that user writes without modification.
    /// - `OneOf`, `AnyOf`, `Order` must return spaces separated ids(first elems in tuples) in the same
    ///   order as it's displayed on the user screen at the end.
    fn interact(&mut self, items: Vec<InterationItem>) -> Vec<String>;
}

pub trait Task<'a>: Serialize + Deserialize<'a> {
    type SharedState: SharedState<'a>;

    fn get_desctiption(&self) -> &str;

    fn next_repetition(&self, _retrievability_goal: f64) -> SystemTime;
    fn complete(
        self,
        shared_state: &mut Self::SharedState,
        interaction: &mut impl UserInteraction,
    ) -> (Self, Feedback);
}

pub trait SharedState<'a>: Default + Serialize + Deserialize<'a> {}
impl<'a> SharedState<'a> for () {}

pub trait SharedStateExt<'a>: SharedState<'a> {
    fn optimize(&mut self);
}
