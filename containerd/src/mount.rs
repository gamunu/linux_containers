/// Mount is the lingua franca of containerd. A mount represents a
/// serialized mount syscall. Components either emit or consume mounts.
pub struct Mount {
	// Type specifies the host-specific of the mount.
	mount_type: String,
	// Source specifies where to mount from. Depending on the host system, this
	// can be a source path or device.
	source: String,
	// Options contains zero or more fstab-style mount options. Typically,
	// these are platform specific.
	options: Vec<String>,
}
