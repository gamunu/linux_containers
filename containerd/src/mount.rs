pub mod losetup;

use loopdev::LoopControl;
use nix::unistd;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

static ALLOWED_HELPER_BINARIES: [&'static str; 2] = ["mount.fuse", "mount.fuse3"];
static PAGE_SIZE: usize = 4096;

/// Mount is the lingua franca of containerd. A mount represents a
/// serialized mount syscall. Components either emit or consume mounts.
pub struct Mount {
    // Type specifies the host-specific of the mount.
    mount_type: String,
    // Source specifies where to mount from. Depending on the host system, this
    // can be a source path or device.
    source: PathBuf,
    // Options contains zero or more fstab-style mount options. Typically,
    // these are platform specific.
    options: Vec<String>,
}

impl Mount {
    pub fn all(mounts: &mut [Mount], target: &str) -> Result<(), String> {
        todo!()
    }

    /// mount to the provided target path.
    ///
    /// If m.Type starts with "fuse." or "fuse3.", "mount.fuse" or "mount.fuse3"
    /// helper binary is called.
    pub fn mount(&self, target: &str) -> Result<(), String> {
        for binary in ALLOWED_HELPER_BINARIES {
            // ALLOWED_HELPER_BINARIES = "mount.fuse", typePrefix = "fuse."
            let type_prefix = binary.strip_prefix("mount.").unwrap();
            if type_prefix.starts_with(&self.mount_type) {
                return self.mount_with_helper(binary, type_prefix, target);
            }
        }

        let mut chdir: PathBuf = PathBuf::from("");
        // TODO: This is pretty bad .clone(). in rust terms. Need to find a better way to implement
        // this logic
        let mut options: Vec<String> = self.options.clone();

        // avoid hitting one page limit of mount argument buffer
        //
        // NOTE: 512 is a buffer during pagesize check.
        if self.mount_type == "overlay" && self.option_size() >= PAGE_SIZE - 512 {
            (chdir, options) = self
                .compact_lower_dir_option(&self.options)
                .unwrap_or((PathBuf::from(""), self.options.clone()))
        }

        let (flags, data, losetup) = self.parse_mount_options(&options);

        let pagesize = unistd::sysconf(unistd::SysconfVar::PAGE_SIZE)
            .unwrap()
            .unwrap() as usize;

        if data.len() > pagesize {
            return Err("mount options is too long".to_string());
        }

        // propagation types.
        let ptypes: u64 = libc::MS_SHARED
            | libc::MS_PRIVATE
            | libc::MS_SLAVE
            | libc::MS_UNBINDABLE;

        // Ensure propagation type change flags aren't included in other calls.
        let oflags = flags ^ !ptypes;

        // In the case of remounting with changed data (data != ""), need to call mount (moby/moby#34077).
        if (flags & libc::MS_REMOUNT) == 0 || data != "" {
            // Initial call applying all non-propagation flags for mount
            // or remount with changed data
            if losetup {
                //setup loop hear
                let read_only: bool = oflags&libc::MS_RDONLY == libc::MS_RDONLY;
                losetup::setup_loop(&self.source, read_only, true);
            }
        }

        Ok(())
    }

    fn mount_with_helper(
        &self,
        helper_binary: &str,
        type_prefix: &str,
        target: &str,
    ) -> Result<(), String> {
        todo!()
    }

    /// option_size returns the byte size of options of mount.
    fn option_size(&self) -> usize {
        let mut size: usize = 0;

        for opt in &self.options {
            size += opt.len();
        }
        return size;
    }

    /// compact_lower_dir_option updates overlay lowdir option and returns the common
    /// dir among all the lowdirs.
    fn compact_lower_dir_option(&self, opts: &Vec<String>) -> Option<(PathBuf, Vec<String>)> {
        let (idx, dirs) = match self.find_overlay_lower_dirs(opts) {
            Some((idx, dirs)) => (idx, dirs),
            None => {
                // no need to compact if there is only one lowerdir
                return None;
            }
        };

        // find out common dir
        let common_dir = self.longest_common_prefix(&dirs);
        if common_dir.is_empty() {
            return None;
        }

        // NOTE: the snapshot id is based on digits.
        // in order to avoid to get snapshots/x, should be back to parent dir.
        // however, there is assumption that the common dir is ${root}/io.containerd.v1.overlayfs/snapshots.
        let common_path = match Path::new(&common_dir).parent() {
            Some(x) => x.join("/"),
            // returns None if path terminates in root
            None => return None,
        };

        let mut new_dirs: Vec<String> = Vec::new();
        for dir in dirs {
            new_dirs.push(dir[common_dir.chars().count()..].to_string());
        }

        let mut new_opts = [&opts[..idx], &opts[idx + 1..]].concat();
        new_opts.push(format!("lowerdir={}", new_dirs.join(":")));

        Some((common_path, new_opts))
    }

    /// findOverlayLowerdirs returns the index of lowerdir in mount's options and
    /// all the lowerdir target.
    fn find_overlay_lower_dirs(&self, opts: &Vec<String>) -> Option<(usize, Vec<String>)> {
        for (i, opt) in opts.iter().enumerate() {
            match opt.strip_prefix("lowerdir=") {
                Some(target) => {
                    let vec = target.split(":").map(|s| s.to_string()).collect();
                    return Some((i, vec));
                }
                _ => {}
            }
        }
        None
    }

    /// longest_common_prefix finds the longest common prefix in the string slice.
    fn longest_common_prefix(&self, strs: &Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        } else if strs.len() == 1 {
            return strs[0].to_string();
        }

        let min_str = &strs[0];

        // find out the min/max value by alphabetical order
        let (mut min, mut max) = (min_str, min_str);

        for str in &strs[1..] {
            if min.gt(str) {
                min = str;
            }
            if max.le(str) {
                max = str;
            }
        }

        let min_count = min.chars().count();
        let max_count = max.chars().count();

        // find out the common part between min and max
        for x in min_count..max_count {
            if min.chars().nth(x).unwrap() != max.chars().nth(x).unwrap() {
                return min[..x].to_string();
            }
        }
        return min.to_string();
    }

    /// parse_mount_options takes fstab style mount options and parses them for
    /// use with a standard mount() syscall
    fn parse_mount_options(&self, options: &Vec<String>) -> (u64, String, bool) {
        let loop_opt = "loop".to_string();
        let mut flag: u64 = 0;
        let mut data: Vec<String> = Vec::new();
        let mut losetup: bool = false;

        struct Flag {
            clear: bool,
            flag: u64,
        }

        impl Flag {
            /// Creates a new Flags.
            fn new(clear: bool, flag: u64) -> Flag {
                Flag {
                    clear: clear,
                    flag: flag,
                }
            }
        }

        let flags = HashMap::from([
            ("async", Flag::new(true, libc::MS_SYNCHRONOUS)),
            ("atime", Flag::new(true, libc::MS_NOATIME)),
            ("bind", Flag::new(false, libc::MS_BIND)),
            ("defaults", Flag::new(false, 0)),
            ("dev", Flag::new(false, libc::MS_NODEV)),
            ("diratime", Flag::new(false, libc::MS_NODIRATIME)),
            ("dirsync", Flag::new(false, libc::MS_DIRSYNC)),
            ("exec", Flag::new(false, libc::MS_NOEXEC)),
            ("mand", Flag::new(false, libc::MS_MANDLOCK)),
            ("noatime", Flag::new(false, libc::MS_NOATIME)),
            ("nodev", Flag::new(false, libc::MS_NODEV)),
            ("nodiratime", Flag::new(false, libc::MS_NODIRATIME)),
            ("noexec", Flag::new(false, libc::MS_NOEXEC)),
            ("nomand", Flag::new(false, libc::MS_MANDLOCK)),
            ("norelatime", Flag::new(false, libc::MS_RELATIME)),
            ("nostrictatime", Flag::new(false, libc::MS_STRICTATIME)),
            ("nosuid", Flag::new(false, libc::MS_NOSUID)),
            (
                "rbind",
                Flag::new(false, libc::MS_BIND | libc::MS_REC),
            ),
            ("relatime", Flag::new(false, libc::MS_RELATIME)),
            ("remount", Flag::new(false, libc::MS_REMOUNT)),
            ("ro", Flag::new(false, libc::MS_RDONLY)),
            ("rw", Flag::new(false, libc::MS_RDONLY)),
            ("strictatime", Flag::new(false, libc::MS_STRICTATIME)),
            ("suid", Flag::new(false, libc::MS_NOSUID)),
            ("sync", Flag::new(false, libc::MS_SYNCHRONOUS)),
        ]);

        for opt in options {
            // If the option does not exist in the flags table or the flag
            // is not supported on the platform,
            // then it is a data value for a specific fs type
            let f = flags.get(opt.as_str());

            if f.is_some() && f.unwrap().flag != 0 {
                let f = f.unwrap();
                if f.clear {
                    flag = !(f.flag ^ f.flag);
                } else {
                    flag |= f.flag;
                }
            } else if opt.eq(&loop_opt) {
                losetup = true;
            } else {
                data.push(opt.clone());
            }
        }

        (flag, data.join(","), losetup)
    }
}
