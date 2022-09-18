pub mod events {
    include!(concat!(env!("OUT_DIR"), "/containerd.api.events.rs"));
}

pub mod types {
    include!(concat!(env!("OUT_DIR"), "/containerd.api.types.rs"));
}

pub mod sandbox {
    pub mod runtime {
        pub mod v1 {
            include!(concat!(
                env!("OUT_DIR"),
                "/containerd.api.runtime.sandbox.v1.rs"
            ));
        }
    }
}

pub mod services {
    pub mod containers {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.containers.v1.rs"));
        }
    }
    pub mod content {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.content.v1.rs"));
        }
    }
    pub mod diff {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.diff.v1.rs"));
        }
    }
    pub mod events {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.events.v1.rs"));
        }
    }
    pub mod images {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.images.v1.rs"));
        }
    }
    pub mod introspection {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.introspection.v1.rs"));
        }
    }
    pub mod leases {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.leases.v1.rs"));
        }
    }
    pub mod namespaces {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.namespaces.v1.rs"));
        }
    }
    pub mod sandbox {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.sandbox.v1.rs"));
        }
    }
    pub mod snapshots {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.snapshots.v1.rs"));
        }
    }
    pub mod tasks {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.tasks.v1.rs"));
        }
    }
    pub mod ttrpc {
        pub mod events {
            pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.events.ttrpc.v1.rs"));
            }
        }
    }
    pub mod version {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/containerd.api.services.version.v1.rs"));
        }
    }
}

pub mod task {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/containerd.api.task.v2.rs"));
    }
}

pub mod v1 {
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/containerd.api.v1.types.rs"));
    }
}
