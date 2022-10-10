// Adaptor specifies the mapping of fieldpaths to a type. For the given field
// path, the value and whether it is present should be returned. The mapping of
// the fieldpath to a field is deferred to the adaptor implementation, but
// should generally follow protobuf field path/mask semantics.
pub trait Adaptor {
    fn field<'a>(&self, fieldpath: Vec<&'a str>) -> (&'a str, bool);
}

// AdapterFunc allows implementation specific matching of fieldpaths
pub type AdaptorFunc = fn(fieldpath: Vec<&str>) -> (&str, bool);

impl Adaptor for AdaptorFunc {
    // Field returns the field name and true if it exists
    fn field<'a>(&self, fieldpath: Vec<&'a str>) -> (&'a str, bool) {
        self(fieldpath)
    }
}
