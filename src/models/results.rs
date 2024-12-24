use std::time::Duration;

pub struct DispatcherResult {
    pub duration: Duration
}

impl DispatcherResult {
    pub fn new() -> DispatcherResult {
        DispatcherResult {
            duration: Duration::new(0, 0)
        }
    }
}
