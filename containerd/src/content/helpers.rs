use crate::content::Provider;
use crate::error::{Error, ErrorKind, Result};
use oci_spec::image;

// read_blob retrieves the entire contents of the blob from the provider.
//
// Avoid using this for large blobs, such as layers.
fn read_blob(provider: Box<dyn Provider>, desc: image::Descriptor) -> Result<u64> {
    let ra = match provider.reader_at(&desc) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    todo!()
}
