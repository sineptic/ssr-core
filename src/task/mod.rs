use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::Duration};

pub mod level;

#[derive(Debug, Clone)]
pub enum Feedback<'a, I: Iterator<Item = &'a String>> {
    CorrectAnswer,
    WrongAnswer {
        correct_answers: I,
        explanation: &'a Option<String>,
    },
}

pub trait UserInteraction {
    fn get_string(&mut self, title: impl Display) -> String;
    fn select_item(&mut self, title: Option<impl Display>, items: &[impl Display]) -> usize;
    fn select_multipe(
        &mut self,
        title: Option<impl Display>,
        items: &[impl Display],
    ) -> Box<[bool]>;
}

pub trait Task<'a>: Serialize + Deserialize<'a> {
    fn new(
        description: String,
        correct_answers: impl IntoIterator<Item = String>,
        explanation: Option<String>,
    ) -> Self;

    fn get_desctiption(&self) -> &str;

    fn until_next_repetition(&self) -> Duration;

    fn complete(
        &mut self,
        interaction: impl UserInteraction,
    ) -> Feedback<impl Iterator<Item = &String>>;
}
