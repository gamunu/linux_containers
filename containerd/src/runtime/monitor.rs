use super::task::Task;
use crate::error::Result;
use std::collections::HashMap;

/// TaskMonitor provides an interface for monitoring of containers within containerd
pub trait TaskMonitor {
    // monitor adds the provided container to the monitor.
    // Labels are optional (can be nil) key value pairs to be added to the metrics namespace.
    fn monitor(&self, task: &dyn Task, labels: &mut HashMap<String, String>) -> Result<String>;
    fn stop(&self, task: &dyn Task) -> Result<String>;
}

struct NoopTaskMonitor {}

impl TaskMonitor for NoopTaskMonitor {
    fn monitor(&self, task: &dyn Task, labels: &mut HashMap<String, String>) -> Result<String> {
        todo!()
    }

    fn stop(&self, task: &dyn Task) -> Result<String> {
        todo!()
    }
}

struct MultiTaskMonitor {
    monitors: Vec<Box<dyn TaskMonitor>>,
}

impl TaskMonitor for MultiTaskMonitor {
    fn monitor(&self, task: &dyn Task, labels: &mut HashMap<String, String>) -> Result<String> {
        for monitor in self.monitors.iter() {
            match monitor.monitor(task, labels) {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(())
    }

    fn stop(&self, task: &dyn Task) -> Result<String> {
        for monitor in self.monitors.iter() {
            match monitor.stop(task) {
                Err(e) => return Err(e),
                _ => {}
            }
        }
        Ok(())
    }
}
