use data_encoding::HEXUPPER;
use ring::digest;
use std::io::Read;

// ImageDigest allows simple protection of hex formatted digest strings, prefixed
// by their algorithm. Strings of type Digest have some guarantee of being in
// the correct format and it provides quick access to the components of a
// digest string.
//
// The following is an example of the contents of Digest types:
//
// 	sha256:7173b809ca12ec5dee4506cd86be934c4596dd234ee82c0662eac04a8c2c71dc
//
// This allows to abstract the digest behind this type and work only in those
// terms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageDigest {
    pub algorithm: String,
    pub digest: String,
}

impl ImageDigest {
    pub fn new_sha256<R: Read>(mut reader: R) -> Result<ImageDigest, String> {
        let mut context: digest::Context = digest::Context::new(&digest::SHA256);
        let mut buffer: [u8; 1024] = [0; 1024];

        loop {
            let count: usize = match reader.read(&mut buffer) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };
            if count == 0 {
                break;
            }
            context.update(&buffer[..count]);
        }
        let digest: String = HEXUPPER.encode(context.finish().as_ref());

        Ok(ImageDigest { algorithm: "sha256".to_string(), digest: digest })
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.algorithm, self.digest)
    }
}
