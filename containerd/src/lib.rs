mod runtime;
mod containers;
pub mod api;
pub mod mount;

//TODO: Find out how we can include google/rpc/status.proto
pub mod plugin {
    pub mod rpc {
        include!(concat!(env!("OUT_DIR"), "/containerd.plugin.rpc.rs"));
    }
}

// Version holds the complete version number. Filled in at linking time.
pub static VERSION: &'static str = "1.6.0+unknown";

// Revision is filled with the VCS (e.g. git) revision being used to build
// the program at linking time.
pub static REVISION: &'static str = "";
