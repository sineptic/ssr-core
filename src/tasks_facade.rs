use crate::task::{Feedback, Task, UserInteraction};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no task to complete, time until next repetition: {}s", time_until_next_repetition.as_secs())]
    NoTaskToComplete {
        time_until_next_repetition: Duration,
    },
    #[error("tasks facade is empty")]
    NoTask,
}

pub trait TasksFacade<'a, T: Task<'a>>: Serialize + Deserialize<'a> {
    fn new(name: String) -> Self;
    fn get_name(&self) -> &str;
    fn tasks_total(&self) -> usize;
    fn tasks_to_complete(&self) -> usize;
    fn complete_task(&mut self, interaction: impl UserInteraction) -> Result<Feedback, Error>;
    fn insert(&mut self, task: T);

    fn iter<'t>(&'t self) -> impl Iterator<Item = &'t T>
    where
        T: 't;
    /// Remove task.
    /// Returns whether such an element was present.
    fn remove(&mut self, task: &T) -> bool;
}
