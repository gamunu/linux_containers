mod runtime;
mod mount;
mod containers;

// Version holds the complete version number. Filled in at linking time.
pub static VERSION: &'static str = "1.6.0+unknown";

// Revision is filled with the VCS (e.g. git) revision being used to build
// the program at linking time.
pub static REVISION: &'static str = "";
