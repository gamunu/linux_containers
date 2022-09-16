use crate::types::{Event, EventInfo};

/// Backend defines callbacks that the client of the library needs to implement.
pub trait Backend {
    fn process_event(container_id: String, event: Event, event_info: EventInfo);
}
