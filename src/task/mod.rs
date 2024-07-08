pub mod task_facade;

use crate::level::TaskLevel;
use std::{
    collections::BTreeSet,
    time::{Duration, SystemTime},
};

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

    pub fn get_desctiption(&self) -> &str {
        &self.description
    }

    pub fn until_next_repetition(&self) -> Duration {
        (self.last_repetition_time + self.level.duration())
            .duration_since(SystemTime::now())
            .unwrap_or_default()
    }

    pub fn complete(
        &mut self,
        respondent: impl FnOnce(&String) -> String,
    ) -> Option<&Option<String>> {
        match self
            .correct_answers
            .contains(&respondent(&self.description))
        {
            true => {
                self.level.success();
                self.last_repetition_time = SystemTime::now();
                None
            }
            false => {
                self.level.failure();
                self.last_repetition_time = SystemTime::now();
                Some(&self.explanation)
            }
        }
    }
}
