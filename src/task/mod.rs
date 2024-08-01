use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::SystemTime};

pub mod level;

#[derive(Debug, Clone)]
pub enum Feedback {
    CorrectAnswer,
    WrongAnswer {
        correct_answers: Vec<String>,
        explanation: Option<String>,
    },
}

pub trait UserInteraction {
    fn get_string(&mut self, title: Option<impl Display>, prompt: impl Display) -> String;
    fn select_item(&mut self, title: Option<impl Display>, items: &[impl Display]) -> usize;
    fn select_multipe(
        &mut self,
        title: Option<impl Display>,
        items: &[impl Display],
    ) -> Box<[bool]>;
}

pub trait Task<'a>: Serialize + Deserialize<'a> {
    type SharedState: SharedState<'a>;

    fn get_desctiption(&self) -> &str;

    fn next_repetition(&self, _retrievability_goal: f64) -> SystemTime;

    fn complete(
        self,
        shared_state: &mut Self::SharedState,
        interaction: impl UserInteraction,
    ) -> (Self, Feedback);
}

pub trait SharedState<'a>: Default + Serialize + Deserialize<'a> {}

pub trait SharedStateExt<'a>: SharedState<'a> {
    fn optimize(&mut self);
}
