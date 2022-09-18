use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {

    //build plugins 
    prost_build::Config::new().compile_protos(
        &[
            "proto/plugin/rpc/status.proto",
        ],
        &["./proto/"],
    )?;

    prost_build::Config::new().compile_protos(
        &[
            "proto/api/types/descriptor.proto",
            "proto/api/types/metrics.proto",
            "proto/api/types/mount.proto",
            "proto/api/types/platform.proto",
            "proto/api/types/sandbox.proto",
            "proto/api/types/task/task.proto",
        ],
        &["./proto/"],
    )?;

    prost_build::Config::new().compile_protos(
        &[
            "proto/api/events/container.proto",
            "proto/api/events/content.proto",
            "proto/api/events/image.proto",
            "proto/api/events/namespace.proto",
            "proto/api/events/snapshot.proto",
            "proto/api/events/task.proto",
        ],
        &["./proto/"],
    )?;

    prost_build::Config::new().compile_protos(
        &[
            "proto/api/runtime/sandbox/v1/sandbox.proto",
            "proto/api/runtime/task/v2/shim.proto",
        ],
        &["./proto/"],
    )?;


    prost_build::Config::new().compile_protos(
        &[
            "proto/api/services/containers/v1/containers.proto",
            "proto/api/services/content/v1/content.proto",
            "proto/api/services/diff/v1/diff.proto",
            "proto/api/services/events/v1/events.proto",
            "proto/api/services/images/v1/images.proto",
            "proto/api/services/introspection/v1/introspection.proto",
            "proto/api/services/leases/v1/leases.proto",
            "proto/api/services/namespaces/v1/namespace.proto",
            "proto/api/services/sandbox/v1/sandbox.proto",
            "proto/api/services/snapshots/v1/snapshots.proto",
            "proto/api/services/tasks/v1/tasks.proto",
            "proto/api/services/ttrpc/events/v1/events.proto",
            "proto/api/services/version/v1/version.proto",
        ],
        &["./proto/"],
    )?;

    Ok(())
}
