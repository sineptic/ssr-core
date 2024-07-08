use crate::level::TaskLevel;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    time::{Duration, SystemTime},
};

pub mod task_facade;

#[derive(Debug, Clone)]
pub enum Feedback<'a, I: Iterator<Item = &'a String>> {
    CorrectAnswer,
    WrongAnswer {
        correct_answers: I,
        explanation: &'a Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "Level: TaskLevel"))]
pub struct Task<Level>
where
    Level: TaskLevel,
{
    level: Level,
    last_repetition_time: SystemTime,
    description: String,
    correct_answers: BTreeSet<String>,
    explanation: Option<String>,
}

impl<Level> Task<Level>
where
    Level: TaskLevel,
{
    /// O(1)
    pub fn new(
        description: String,
        correct_answers: BTreeSet<String>,
        explanation: Option<String>,
    ) -> Self {
        Self {
            level: Level::default(),
            last_repetition_time: SystemTime::now(),
            description,
            correct_answers,
            explanation,
        }
    }

    /// O(1)
    pub fn get_desctiption(&self) -> &str {
        &self.description
    }

    /// O(1)
    pub fn until_next_repetition(&self) -> Duration {
        (self.last_repetition_time + self.level.duration())
            .duration_since(SystemTime::now())
            .unwrap_or_default()
    }

    /// O(log(correct_answers))
    pub fn complete(
        &mut self,
        respondent: impl FnOnce(&String) -> String,
    ) -> Feedback<impl Iterator<Item = &String>> {
        self.last_repetition_time = SystemTime::now();
        match self
            .correct_answers
            .contains(&respondent(&self.description))
        {
            true => {
                self.level.success();
                Feedback::CorrectAnswer
            }
            false => {
                self.level.failure();
                Feedback::WrongAnswer {
                    correct_answers: self.correct_answers.iter(),
                    explanation: &self.explanation,
                }
            }
        }
    }
}
