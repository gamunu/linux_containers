mod handlers;

use super::content;
use oci_spec::image;
use std::collections::HashMap;

// Image provides the model for how containerd views container images.
pub struct Image {
    // name of the image.
    //
    // To be pulled, it must be a reference compatible with resolvers.
    //
    // This field is required.
    name: String,

    // labels provide runtime decoration for the image record.
    //
    // There is no default behavior for how these labels are propagated. They
    // only decorate the static metadata object.
    //
    // This field is optional.
    labels: HashMap<String, String>,

    // target describes the root content for this image. Typically, this is
    // a manifest, index or manifest list.
    target: image::Descriptor,

    create_at: time::Time,

    updated_at: time::Time,
}

//impl Image {
//    fn config(provider: content::Provider)
//}

// manifest resolves a manifest from the image for the given platform.
//
// When a manifest descriptor inside of a manifest index does not have
// a platform defined, the platform from the image config is considered.
//
// If the descriptor points to a non-index manifest, then the manifest is
// unmarshalled and returned without considering the platform inside of the
// config.
pub fn manifest(
    provider: Box<dyn content::Provider>,
    image: image::Descriptor,
) -> Result<image::ImageManifest, String> {
    let limit: i32 = 1;
    let was_index: bool = false;

    let walk = |desc: image::Descriptor| -> Result<Vec<image::Descriptor>, String> {
        //   match desc.media_type() {
        //      oci_spec::image::MediaType::ImageManifest => {
        todo!()
        //      }

        //  }
    };
    //handlers::walk()
    todo!()
}
