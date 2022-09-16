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
    fn as_str(&self) -> &'static str {
        match self {
            EventTopic::TaskCreate => "/tasks/create",
            EventTopic::TaskStart => "/tasks/start",
            EventTopic::TaskOOM =>  "/tasks/oom",
            EventTopic::TaskExit => "/tasks/exit",
            EventTopic::TaskDelete => "/tasks/delete",
            EventTopic::TaskExecAdded => "/tasks/exec-added",
            EventTopic::TaskExecStarted =>  "/tasks/exec-started",
            EventTopic::TaskPaused => "/tasks/paused",
            EventTopic::TaskResumed => "/tasks/resumed",
            EventTopic::TaskCheckpointed => "/tasks/checkpointed",
            EventTopic::TaskUnknown => "/tasks/?",
        }
    }
}
