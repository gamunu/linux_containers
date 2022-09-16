use core::str;
use std::collections::HashMap;

use time::Time;

/// TaskInfo provides task specific information
pub struct TaskInfo {
    id:String,
    runtime: String,
    spec: Vec<u8>,
    namespace: String,
}

/// Process is a runtime object for an executing process inside a container
pub trait Process {
    // id of the process
    fn id(&self) -> String;
    // state returns the process state
    fn state(&self) -> Result<State, String>;
    // kill signals a container
    fn kill(&self, signal: u32, all: bool) -> Result<(), String>;
    // resize_pty resizes the processes pty/console
    fn resize_pty(&self, size: ConsoleSize) -> Result<(), String>;
    // close_io closes the processes IO
    fn close_io(&self) -> Result<(), String>;
    // start the container's user defined process
    fn start(&self) -> Result<(), String>;
    // wait for the process to exit
    fn wait(&self) -> Result<super::Exit, String>;
}

/// ExecProcess is a process spawned in container via Task.Exec call.
/// The only difference from a regular `Process` is that exec process can delete self,
/// while task process requires slightly more complex logic and needs to be deleted through the task manager.
pub trait ExecProcess: Process {
    // delete deletes the process
    fn delete(&self) -> Result<super::Exit, String>;
}

/// Task is the runtime object for an executing container
pub trait Task {
    // pid of the process
    fn pid(&self) -> Result<u32, String>;
    // namespace that the task exists in
    fn namespace(&self) -> String;
    // pause pauses the container process
    fn pause(&self) -> Result<(), String>;
    // resume unpauses the container process
    fn resume(&self) -> Result<(), String>;
    // exec adds a process into the container
    fn exec(&self, id: &str, opts: ExecOpts) -> Result<Box<dyn ExecProcess>, String>;
    // pids returns all pids
    fn pids(&self) -> Result<Vec<ProcessInfo>, String>;
    // check_point checkpoints a container to an image with live system data
    fn check_point(&self, path: &str, opts: *mut prost_types::Any) -> Result<(), String>;
    // update sets the provided resources to a running task
    fn update(&self, resources: *mut prost_types::Any, annotations: HashMap<String, String>) -> Result<(), String>;
    // process returns a process within the task for the provided id
    fn process(&self, id: &str) -> Result<Box<dyn ExecProcess>, String>;
    // stats returns runtime specific metrics for a task
    fn stats(&self) -> Result<*mut prost_types::Any, String>;
}

/// ExecOpts provides additional options for additional processes running in a task
pub struct ExecOpts {
    spec: *mut prost_types::Any,
    io: super::IO,
}

/// ConsoleSize of a pty or windows terminal
pub struct ConsoleSize {
    width: u32,
    height: u32,
}

/// Status is the runtime status of a task and/or process
pub enum Status {
    // CreatedStatus when a process has been created
    CreatedStatus,
    // RunningStatus when a process is running
    RunningStatus,
    // StoppedStatus when a process has stopped
    StoppedStatus,
    // DeletedStatus when a process has been deleted
    DeletedStatus,
    // PausedStatus when a process is paused
    PausedStatus,
    // PausingStatus when a process is currently pausing
    PausingStatus,
}

/// State information for a process
pub struct State {
    // status is the current status of the container
    status: Status,
    // pid is the main process id for the container
    pid: u32,
    // exit_status of the process
    // Only valid if the Status is Stopped
    exit_status: u32,
    // ExitedAt is the time at which the process exited
    // Only valid if the Status is Stopped
    exited_at: Time,
    stdin: String,
    stdout: String,
    stderr: String,
    terminal: bool,
}

/// ProcessInfo holds platform specific process information
pub struct ProcessInfo {
    // pid is the process ID
    pid: u32,
    // info includes additional process information
    // info varies by platform
    info: prost_types::Any,
}
