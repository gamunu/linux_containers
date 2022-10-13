use crate::error::{Error, ErrorKind, Result};
use filetime::FileTime;
use image_digest::ImageDigest;
use log::warn;
use std::{collections::HashMap, fs, io, ops::Not, path::Path, path::PathBuf, time::SystemTime};

// LabelStore is used to store mutable labels for digests
pub trait LabelStore {
    // get returns all the labels for the given digest
    fn get(&self, digest: &ImageDigest) -> Result<HashMap<&str, &str>>;
    // set sets all the labels for a given digest
    fn set(&self, digest: &ImageDigest, labels: &mut HashMap<&str, &str>) -> Result<()>;
    // update replaces the given labels for a digest,
    // a key with an empty value removes a label.
    fn update(&self, digest: &ImageDigest, labels: &mut HashMap<&str, &str>) -> Result<()>;
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
    fn new<P: AsRef<Path>>(root: P) -> Result<Store> {
        let root_path: &Path = root.as_ref();
        match fs::create_dir(root_path.join("ingest")) {
            Ok(_) => {}
            Err(e) => return Err(Error::new(ErrorKind::IOError, e)),
        };

        Ok(Store {
            root: root_path.to_path_buf(),
            ls: None,
        })
    }

    fn new_with_label_store<P: AsRef<Path>>(root: P, ls: Box<dyn LabelStore>) -> Result<Store> {
        let root_path: &Path = root.as_ref();
        match fs::create_dir(root_path.join("ingest")) {
            Ok(_) => {}
            Err(e) => return Err(Error::new(ErrorKind::IOError, e)),
        };

        Ok(Store {
            root: root_path.to_path_buf(),
            ls: Some(ls),
        })
    }

    // reader_at returns an io.ReaderAt for the blob.
    fn reader_at(&self, digest: &image_digest::ImageDigest) -> Result<fs::File> {
        let path: PathBuf = self.blob_path(digest.clone());
        let path_str: &str = path.as_os_str().to_str().unwrap();

        let file: fs::File = match fs::OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::IOError,
                    format!("blob {} expected at {}: {}", digest.digest, path_str, e),
                ));
            }
        };

        Ok(file)
    }

    fn blob_path(&self, digest: image_digest::ImageDigest) -> PathBuf {
        // validate ??
        self.root
            .join("blobs")
            .join(digest.algorithm)
            .join(digest.digest)
    }
}

impl super::Manager for Store {
    fn info(&self, digest: &image_digest::ImageDigest) -> Result<super::Info> {
        let blob_path: PathBuf = self.blob_path(digest.clone());
        let meta: fs::Metadata = match fs::metadata(blob_path) {
            Ok(m) => m,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::IOError,
                    format!("content {}: {}", digest.to_string(), e.to_string()),
                ));
            }
        };

        let mut labels: HashMap<&str, &str> = HashMap::new();

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

    fn update<'a>(
        &self,
        info: &'a super::Info,
        fieldpaths: Vec<&'a str>,
    ) -> Result<super::Info<'a>> {
        if self.ls.is_none() {
            return Err(Error::new(
                ErrorKind::FailedPrecondition,
                "update not supported on immutable content store",
            ));
        }
        let blob_path: PathBuf = self.blob_path(info.digest.clone());

        let meta = match fs::metadata(&blob_path) {
            Ok(m) => m,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("content {}", info.digest.to_string()),
                ));
            }
        };

        let mut all: bool = false;
        let mut labels: HashMap<&str, &str> = HashMap::new();

        if fieldpaths.is_empty().not() {
            for path in fieldpaths {
                if path.starts_with("labels.") {
                    let key = path.trim_start_matches("labels.");
                    labels.insert(key, info.labels[key]);
                    continue;
                }

                match path {
                    "labels" => {
                        all = true;
                        labels = info.labels.clone();
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidArgument,
                            format!(
                                "cannot update {} field on content info {}",
                                path,
                                info.digest.to_string()
                            ),
                        ));
                    }
                }
            }
        } else {
            all = true;
            labels = info.labels.clone()
        }

        if all {
            // we don't have to worry about unwrap
            // as we alredy checked for the None
            match self.ls.as_ref().unwrap().set(&info.digest, &mut labels) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        } else {
            match self.ls.as_ref().unwrap().update(&info.digest, &mut labels) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        let info = super::Info::new(
            info.digest.clone(),
            meta.len(),
            meta.created().unwrap(),
            SystemTime::now(),
            labels,
        );

        let updated_at: FileTime = FileTime::from(info.updated_at);
        match filetime::set_file_mtime(blob_path, updated_at) {
            Ok(_) => {}
            Err(e) => {
                warn!(
                    "could not change access time for {}: {}",
                    info.digest.to_string(),
                    e
                )
            }
        }

        Ok(info)
    }

    fn walk<F: Fn()>(&self, walkfn: F, fs: Vec<String>) -> Result<()> {
        let root: PathBuf = self.root.join("blobs");
        todo!()
    }

    // delete removes a blob by its digest.
    fn delete(&self, digest: &image_digest::ImageDigest) -> io::Result<()> {
        let path: PathBuf = self.blob_path(digest.clone());

        fs::remove_file(path)
    }
}
