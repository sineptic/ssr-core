use crate::task::{Feedback, Task};
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

pub trait TasksFacade<'a, T: Task<'a>>: Serialize + Deserialize<'a> {
    fn get_name(&self) -> &str;
    fn tasks_total(&self) -> usize;
    fn tasks_to_complete(&self) -> usize;
    fn complete_task(
        &mut self,
        respondent: impl FnOnce(&str) -> String,
    ) -> Result<Feedback<impl Iterator<Item = &String>>, Error>;
    fn insert(&mut self, task: T);
    fn take(&mut self, name: String) -> Option<T>;
}
