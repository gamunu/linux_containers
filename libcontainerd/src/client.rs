/// Client provides access to containerd features.
pub trait Client {
    //TODO: continues from here after implementing the continerd
}

pub struct LibClient {
    state_dir: String,
    ns: String,
}
