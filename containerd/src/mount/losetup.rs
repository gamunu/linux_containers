use loopdev::LoopControl;
use nix::ioctl_read;
use nix::mount;
use nix::mount::MsFlags;
use nix::unistd;
use std::collections::HashMap;
use std::os::raw::c_ulong;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::path::PathBuf;
use std::{fs::File, path::PathBuf};

/// LoopParams parameters to control loop device setup
struct LoopParams {
    // Loop device should forbid write
    readonly: bool,
    // Loop device is automatically cleared by kernel when the
    // last opener closes it
    autoclear: bool,
    // Use direct IO to access the loop backing file
    direct: bool,
}

pub fn setup_loop(read_only: bool, auto_clear: bool) -> Result<PathBuf> {
    let lc = match LoopControl::open() {
        Ok(ctrl) => ctrl,
        Err(e) => return Err(format!("could not open loop control: {}", e)),
    };

    let ld = match lc.next_free() {
        Ok(dev) => dev,
        Err(e) => return Err(format!("could not open loop device: {}", e)),
    };

    match ld
        .with()
        .autoclear(auto_clear)
        .read_only(oflags & libc::MS_RDONLY == libc::MS_RDONLY)
        .attach(source)
    {
        Ok(_) => {}
        Err(e) => return Err(format!("could not set loop fd for device: {}", e)),
    };

    return match ld.path() {
        Some(file) => Some(file),
        None => return Err(format("could net get the loop device path")),
    };
}
