use loopdev::LoopControl;
use std::path::PathBuf;

pub fn setup_loop(source: &PathBuf, read_only: bool, auto_clear: bool) -> Result<PathBuf, String> {
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
        .read_only(read_only)
        .attach(source)
    {
        Ok(_) => {}
        Err(e) => return Err(format!("could not set loop fd for device: {}", e)),
    };

    return match ld.path() {
        Some(file) => Ok(file),
        None => return Err(format!("could net get the loop device path")),
    };
}
