use std::time::Duration;

pub trait TaskLevel: Default {
    fn success(&mut self);
    fn failure(&mut self);
    fn duration(&self) -> Duration;
}
