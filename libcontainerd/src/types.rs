use core::str;
use time::Time;

pub enum Event {
    Unknown,
    Exit,
    OOM,
    Create,
    Start,
    ExecAdded,
    ExecStarted,
    Paused,
    Resumed,
}

impl Event {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            Event::Unknown => "unknown",
            Event::Exit => "exit",
            Event::OOM => "oom",
            Event::Create => "create",
            Event::Start => "start",
            Event::ExecAdded => "exec-added",
            Event::ExecStarted => "exec-started",
            Event::Paused => "paused",
            Event::Resumed => "resumed",
        }
    }
}

pub struct EventInfo<'event> {
    container_id: &'event str,
    process_id: &'event str,
    pid: u32,
    exit_code: u32,
    exited_at: Time,
    error: &'event str,
}

pub struct Version {}
