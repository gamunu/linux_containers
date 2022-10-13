use crate::error::Result;
use std::collections::HashMap;
use time::Time;

/// Container represents the set of data pinned by a container. Unless otherwise
/// noted, the resources here are considered in use by the container.
///
/// The resources specified in this object are used to create tasks from the container.
pub struct Container {
    /// id uniquely identifies the container in a namespace.
    ///
    /// This property is required and cannot be changed after creation.
    id: String,

    /// labels provide metadata extension for a container.
    ///
    /// These are optional and fully mutable.
    labels: HashMap<String, String>,

    /// image specifies the image reference used for a container.
    ///
    /// This property is optional and mutable.
    image: String,

    /// runtime specifies which runtime should be used when launching container
    /// tasks.
    ///
    /// This property is required and immutable.
    runtime: RuntimeInfo,

    /// Spec should carry the runtime specification used to implement the
    /// container.
    ///
    /// This field is required but mutable.
    spec: prost_types::Any,

    /// snapshot_key specifies the snapshot key to use for the container's root
    /// filesystem. When starting a task from this container, a caller should
    /// look up the mounts from the snapshot service and include those on the
    /// task create request.
    ///
    /// This field is not required but mutable.
    snapshot_key: String,

    /// snapshotter specifies the snapshotter name used for rootfs
    ///
    /// This field is not required but immutable.
    snapshotter: String,

    /// created_at is the time at which the container was created.
    created_at: Time,

    /// updated_at is the time at which the container was updated.
    updated_at: Time,

    /// extensions stores client-specified metadata
    extensions: HashMap<String, prost_types::Any>,

    /// sandbox_id is an identifier of sandbox this container belongs to.
    ///
    /// This property is optional, but can't be changed after creation.
    sandbox_id: String,
}

/// RuntimeInfo holds runtime specific information
struct RuntimeInfo {
    name: String,
    options: prost_types::Any,
}

/// Store interacts with the underlying container storage
trait Store {
    /// Get a container using the id.
    ///
    /// Container object is returned on success. If the id is not known to the
    /// store, an error will be returned.
    fn get(&self, id: &str) -> Result<Container>;

    /// list returns containers that match one or more of the provided filters.
    fn list(&self, filters: &[&str]) -> Result<Vec<Container>>;

    /// create a container in the store from the provided container.
    fn create(&self, container: Container) -> Result<Vec<Container>>;

    /// update the container with the provided container object. ID must be set.
    ///
    /// If one or more fieldpaths are provided, only the field corresponding to
    /// the fieldpaths will be mutated.
    fn update(&self, container: Container, fieldpaths: &[&str]) -> Result<Vec<Container>>;

    /// delete a container using the id.
    ///
    /// nil will be returned on success. If the container is not known to the
    /// store, ErrNotFound will be returned.
    fn delete(&self, id: &str) -> Result<()>;
}
