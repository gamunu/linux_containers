use std::ops::{Deref, DerefMut};

// Adaptor specifies the mapping of fieldpaths to a type. For the given field
// path, the value and whether it is present should be returned. The mapping of
// the fieldpath to a field is deferred to the adaptor implementation, but
// should generally follow protobuf field path/mask semantics.
pub trait Adaptor {
    fn field(&self, fieldpath: Vec<String>) -> (String, bool);
}

// AdapterFunc allows implementation specific matching of fieldpaths
pub struct AdapterFunc<F: Fn(Vec<String>) -> (String, bool)>(pub F);

impl<F> Adaptor for AdapterFunc<F>
where
    F: Fn(Vec<String>) -> (String, bool),
{
    // Field returns the field name and true if it exists
    fn field<'a>(&self, fieldpath: Vec<String>) -> (String, bool) {
        self(fieldpath)
    }
}

impl<F> Deref for AdapterFunc<F>
where
    F: Fn(Vec<String>) -> (String, bool),
{
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> DerefMut for AdapterFunc<F>
where
    F: Fn(Vec<String>) -> (String, bool),
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
