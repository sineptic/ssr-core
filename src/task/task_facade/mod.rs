use super::{Feedback, Task, TaskLevel};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no task to complete, time until next repetition: {}s", time_until_next_repetition.as_secs())]
    NoTaskToComplete {
        time_until_next_repetition: Duration,
    },
}

pub trait TaskFacade<'a, Level>: Serialize + Deserialize<'a>
where
    Level: TaskLevel,
{
    /// O(1)
    fn get_name(&self) -> &str;
    /// O(1)
    fn tasks_total(&self) -> usize;
    /// O(tasks)
    fn tasks_to_complete(&self) -> usize;
    fn complete_task(
        &mut self,
        respondent: impl FnOnce(&str) -> String,
    ) -> Result<Feedback<impl Iterator<Item = &String>>, Error>;
    /// O(log(tasks))
    fn insert(&mut self, task: Task<Level>);
    /// O(log(tasks))
    fn take(&mut self, name: String) -> Option<Task<Level>>;
}
