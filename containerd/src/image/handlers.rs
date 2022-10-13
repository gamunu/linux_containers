use oci_spec::image;

// ERR_SKIP_DESC is used to skip processing of a descriptor and
// its descendants.
const ERR_SKIP_DESC: &str = "skip descriptor";

// ERR_STOP_HANDLER is used to signify that the descriptor
// has been handled and should not be handled further.
// This applies only to a single descriptor in a handler
// chain and does not apply to descendant descriptors.
const ERR_STOP_HANDLER: &str = "stop handler";

// ERR_EMPTY_WALK is used when the WalkNotEmpty handlers return no
// children (e.g.: they were filtered out).
const ERR_EMPTY_WALK: &str = "image might be filtered out";

// Handler handles image manifests
pub trait Handler {
    fn handle(&self, desc: &image::Descriptor) -> Result<Vec<image::Descriptor>, String>;
}

// handlers returns a handler that will run the handlers in sequence.
//
// A handler may return `ErrStopHandler` to stop calling additional handlers
pub fn handlers<'a>(
    handlers: &'a Vec<Box<dyn Handler>>,
) -> impl Fn(image::Descriptor) -> Result<Vec<image::Descriptor>, String> + 'a {
    move |desc: image::Descriptor| -> Result<Vec<image::Descriptor>, String> {
        let mut children: Vec<image::Descriptor> = Vec::new();

        for handler in handlers {
            let ch = match handler.handle(&desc) {
                Ok(c) => c,
                Err(e) => {
                    if e == ERR_STOP_HANDLER {
                        break;
                    }
                    return Err(e);
                }
            };

            children.extend(ch);
        }

        Ok(children)
    }
}

// walk the resources of an image and call the handler for each. If the handler
// decodes the sub-resources for each image,
//
// This differs from dispatch in that each sibling resource is considered
// synchronously.
pub fn walk(handler: &Box<dyn Handler>, descs: Vec<image::Descriptor>) -> Result<(), String> {
    for desc in descs {
        let children = match handler.handle(&desc) {
            Ok(c) => c,
            Err(e) => {
                if e == ERR_SKIP_DESC {
                    continue; // don't traverse the children.
                }
                return Err(e);
            }
        };

        if children.len() > 0 {
            match walk(&handler, children) {
                Err(e) => return Err(e),
                Ok(_) => {}
            };
        }
    }
    Ok(())
}
