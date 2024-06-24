use super::{Task, TaskLevel};

pub trait TaskFacade<Level>: IntoIterator
where
    Level: TaskLevel,
{
    fn get_name(&self) -> String;
    fn tasks_total(&self) -> usize;
    fn tasks_to_complete(&self) -> usize;
    fn complete_task(
        &mut self,
        respondent: impl FnOnce(&String) -> String,
    ) -> Option<&Option<String>>;
    fn insert(&mut self, task: Task<Level>);
    fn take(&mut self, name: String) -> Option<Task<Level>>;
}
