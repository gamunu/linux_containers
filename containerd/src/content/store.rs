use image_digest::ImageDigest;
use std::{collections::HashMap, fs, io, path::Path, path::PathBuf};

// LabelStore is used to store mutable labels for digests
pub trait LabelStore {
    // get returns all the labels for the given digest
    fn get(&self, digest: &ImageDigest) -> Result<HashMap<String, String>, String>;
    // set sets all the labels for a given digest
    fn set(&self, digest: ImageDigest, labels: &mut HashMap<String, String>) -> Result<(), String>;
    // update replaces the given labels for a digest,
    // a key with an empty value removes a label.
    fn update(
        &self,
        digest: ImageDigest,
        labels: &mut HashMap<String, String>,
    ) -> Result<(), String>;
}

// Store is digest-keyed store for content. All data written into the store is
// stored under a verifiable digest.
//
// Store can generally support multi-reader, single-writer ingest of data,
// including resumable ingest.
pub struct Store {
    pub root: PathBuf,
    ls: Option<Box<dyn LabelStore>>,
}

impl Store {
    // new returns a local content store
    fn new<P: AsRef<Path>>(root: P) -> io::Result<Store> {
        let root_path: &Path = root.as_ref();
        match fs::create_dir(root_path.join("ingest")) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(Store {
            root: root_path.to_path_buf(),
            ls: None,
        })
    }

    fn new_with_label_store<P: AsRef<Path>>(root: P, ls: Box<dyn LabelStore>) -> io::Result<Store> {
        let root_path: &Path = root.as_ref();
        match fs::create_dir(root_path.join("ingest")) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        Ok(Store {
            root: root_path.to_path_buf(),
            ls: Some(ls),
        })
    }
}

impl super::Store for Store {
    fn info(&self, digest: &image_digest::ImageDigest) -> Result<super::Info, String> {
        let blob_path: PathBuf = self.blob_path(digest.clone());
        let meta: fs::Metadata = match fs::metadata(blob_path) {
            Ok(m) => m,
            Err(e) => return Err(format!("content {}: {}", digest.to_string(), e.to_string())),
        };

        let mut labels: HashMap<String, String> = HashMap::new();

        labels = match self.ls.as_ref().unwrap().get(digest) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        Ok(super::Info::new(
            digest.clone(),
            meta.len(),
            meta.created().unwrap(),
            meta.modified().unwrap(),
            labels,
        ))
    }

    // reader_at returns an io.ReaderAt for the blob.
    fn reader_at(&self, digest: &image_digest::ImageDigest) -> Result<fs::File, String> {
        let path: PathBuf = self.blob_path(digest.clone());
        let path_str: &str = path.as_os_str().to_str().unwrap();

        let file: fs::File = match fs::OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(format!("blob {} expected at {}: {}", digest.digest, path_str ,e))
        };

        Ok(file)
    }

    // delete removes a blob by its digest.
    fn delete(&self, digest: &image_digest::ImageDigest) -> Result<(), String> {
        let path: PathBuf = self.blob_path(digest.clone());
        
        fs::remove_file(path)
    }

    fn blob_path(&self, digest: image_digest::ImageDigest) -> PathBuf {
        // validate ??
        self.root
            .join("blobs")
            .join(digest.algorithm)
            .join(digest.digest)
    }
}
