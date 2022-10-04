pub mod store;
use oci_spec::image;
use std::{collections::HashMap, io, time};

// Provider provides a reader interface for specific content
pub trait Provider {
    // reader_at only requires desc.Digest to be set.
    // Other fields in the descriptor may be used internally for resolving
    // the location of the actual data.
    fn reader_at(&self, desc: image::Descriptor) -> io::Result<u64>;
}

// Ingester writes content
trait Ingester {
    fn write() -> io::Result<Box<dyn Writer>>;
}

// Info holds content specific information
//
// TODO(stevvooe): Consider a very different name for this struct. Info is way
// to general. It also reads very weird in certain context, like pluralization.
pub struct Info {
    pub digest: image_digest::ImageDigest,
    pub size: u64,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
    pub labels: HashMap<String, String>,
}

impl Info {
    pub fn new(
        digest: image_digest::ImageDigest,
        size: u64,
        created_at: time::SystemTime,
        updated_at: time::SystemTime,
        labels: HashMap<String, String>,
    ) -> Info {
        Info {
            digest: digest,
            size: size,
            created_at: created_at, //panic TODO: write this in a safe way
            updated_at: updated_at, //panic
            labels: labels,
        }
    }
}

// Status of a content operation
struct Status {
    iref: String,
    offset: u64,
    total: u64,
    expected: image_digest::ImageDigest,
    started_at: time::SystemTime,
    updated_at: time::SystemTime,
}

// Manager provides methods for inspecting, listing and removing content.
trait Manager {
    // Info will return metadata about content available in the content store.
    //
    // If the content is not present, ErrNotFound will be returned.
    fn info(&self, dgst: image_digest::ImageDigest) -> Result<Info, String>;

    // Update updates mutable information related to content.
    // If one or more fieldpaths are provided, only those
    // fields will be updated.
    // Mutable fields:
    //  labels.*
    fn update(&self, info: Info, fieldpaths: Vec<String>) -> Result<Info, String>;

    // Walk will call fn for each item in the content store which
    // match the provided filters. If no filters are given all
    // items will be walked.
    fn walk<F: Fn()>(&self, walkfn: F, filters: Vec<String>) -> Result<(), String>;

    // delete removes the content from the store.
    fn delete(&self, dgst: image_digest::ImageDigest) -> Result<(), String>;
}

// IngestManager provides methods for managing ingests.
trait IngestManager {
    // status returns the status of the provided ref.
    fn status(&self, iref: String) -> Result<Status, String>;
    // list_statuses returns the status of any active ingestions whose ref match the
    // provided regular expression. If empty, all active ingestions will be
    // returned.
    fn list_statuses(&self, filters: Vec<String>) -> Result<Vec<String>, String>;
    // abort completely cancels the ingest operation targeted by ref.
    fn abort(&self, iref: String) -> Result<(), String>;
}

trait Writer {
    // digest may return empty digest or panics until committed.
    fn digest(&self) -> image_digest::ImageDigest;
    
    // commit commits the blob (but no roll-back is guaranteed on an error).
	// size and expected can be zero-value when unknown.
	// Commit always closes the writer, even on error. ??????
	// ErrAlreadyExists aborts the writer.
    fn commit(size: u64, exptected: image_digest::ImageDigest) -> io::Result<()>;

    // status returns the current state of write
    fn status() -> Result<Status, String>;

	// truncate updates the size of the target blob
    fn truncate(size: u64) -> Result<(), String>;
}

// Store combines the methods of content-oriented interfaces into a set that
// are commonly provided by complete implementations.
pub trait Store: Manager + Provider + IngestManager + Ingester {

}