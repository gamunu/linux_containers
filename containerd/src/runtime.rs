pub mod events;
pub mod monitor;
pub mod task;

use super::mount;
use time::Time;

/// IO holds process IO information
pub struct IO {
    stdin: String,
    stdout: String,
    stderr: String,
    terminal: bool,
}

/// CreateOpts contains task creation data
pub struct CreateOpts {
    // spec is the OCI runtime spec
    spec: prost_types::Any,
    // root_fs mounts to perform to gain access to the container's filesystem
    root_fs: Vec<mount::Mount>,
    // IO for the container's main process
    io: IO,
    // checkpoint digest to restore container state
    checkpoint: String,
    // runtime_options for the runtime
    runtime_options: prost_types::Any,
    // task_options received for the task
    task_options: prost_types::Any,
    // runtime name to use (e.g. `io.containerd.NAME.VERSION`).
    // As an alternative full abs path to binary may be specified instead.
    runtime: String,
    // sandbox_id is an optional ID of sandbox this container belongs to
    sandbox_id: String,
}

/// Exit information for a process
pub struct Exit {
    pid: u32,
    status: u32,
    timestamp: Time,
}

/// PlatformRuntime is responsible for the creation and management of
/// tasks and processes for a platform.
pub trait PlatformRuntime {
    // id of the runtime
    fn id() -> String;
    // create creates a task with the provided id and options
    fn create(opts: CreateOpts) -> Result<Box<dyn task::Task>, String>;
    // tasks returns all the current tasks for the runtime.
    // Any container runs at most one task at a time.
    fn task(all: bool) -> Result<Vec<Box<dyn task::Task>>, String>;
    // delete remove a task.
    fn delete(task_id: &str) -> Result<*mut Exit, String>;
}
