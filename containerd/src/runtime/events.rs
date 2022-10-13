pub enum EventTopic {
    // TaskCreate for task create
    TaskCreate,
    // TaskStart for task start
    TaskStart,
    // TaskOOM for task oom
    TaskOOM,
    // TaskExit for task exit
    TaskExit,
    // TaskDelete for task delete
    TaskDelete,
    // TaskExecAdded for task exec create
    TaskExecAdded,
    // TaskExecStarted for task exec start
    TaskExecStarted,
    // TaskPaused for task pause
    TaskPaused,
    // TaskResumed for task resume
    TaskResumed,
    // TaskCheckpointed for task checkpoint
    TaskCheckpointed,
    // TaskUnknown for unknown task events
    TaskUnknown,
}

impl EventTopic {
    fn as_str<'a>(&self) -> &'a str {
        use EventTopic::*;

        match self {
            TaskCreate => "/tasks/create",
            TaskStart => "/tasks/start",
            TaskOOM => "/tasks/oom",
            TaskExit => "/tasks/exit",
            TaskDelete => "/tasks/delete",
            TaskExecAdded => "/tasks/exec-added",
            TaskExecStarted => "/tasks/exec-started",
            TaskPaused => "/tasks/paused",
            TaskResumed => "/tasks/resumed",
            TaskCheckpointed => "/tasks/checkpointed",
            TaskUnknown => "/tasks/?",
        }
    }
}
