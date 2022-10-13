use core::marker::PhantomData;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[repr(transparent)]
pub(super) struct Repr(PhantomData<ErrorData<Box<Custom>>>);

// All the types `Repr` stores internally are Send + Sync, and so is it.
unsafe impl Send for Repr {}
unsafe impl Sync for Repr {}

pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self::_new(kind, error.into())
    }

    pub fn other<E>(error: E) -> Error
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self::_new(ErrorKind::Other, error.into())
    }

    fn _new(kind: ErrorKind, error: Box<dyn error::Error + Send + Sync>) -> Error {
        Error {
            repr: Repr::new_custom(Box::new(Custom { kind, error })),
        }
    }

    #[inline]
    pub(crate) const fn from_static_message(msg: &'static SimpleMessage) -> Error {
        Self {
            repr: Repr::new_simple_message(msg),
        }
    }
}

impl From<ErrorKind> for Error {
    /// Converts an [`ErrorKind`] into an [`Error`].
    ///
    /// This conversion creates a new error with a simple representation of error kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Error, ErrorKind};
    ///
    /// let not_found = ErrorKind::NotFound;
    /// let error = Error::from(not_found);
    /// assert_eq!("entity not found", format!("{error}"));
    /// ```
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error {
            repr: Repr::new_simple(kind),
        }
    }
}

impl Repr {
    pub(super) fn new(dat: ErrorData<Box<Custom>>) -> Self {
        match dat {
            ErrorData::Simple(kind) => Self::new_simple(kind),
            ErrorData::SimpleMessage(simple_message) => Self::new_simple_message(simple_message),
            ErrorData::Custom(b) => Self::new_custom(b),
        }
    }

    #[inline]
    pub(super) fn new_custom(b: Box<Custom>) -> Self {
        Self(PhantomData)
    }

    #[inline]
    pub(super) fn new_simple(kind: ErrorKind) -> Self {
        Self(PhantomData)
    }

    #[inline]
    pub(super) const fn new_simple_message(m: &'static SimpleMessage) -> Self {
        // Safety: References are never null.
        Self(PhantomData)
    }
}

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    Unknown,
    InvalidArgument,
    NotFound,
    AlreadyExists,
    FailedPrecondition,
    Unavailable,
    NotImplemented,
    IOError,
    Other,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        use ErrorKind::*;

        match *self {
            Unknown => "unknown",
            InvalidArgument => "invalid argument",
            NotFound => "not found",
            AlreadyExists => "already exists",
            FailedPrecondition => "failed precondition",
            Unavailable => "unavailable",
            NotImplemented => "not implemented",
            Other => "other error",
            IOError => "IO error",
        }
    }
}

impl fmt::Display for ErrorKind {
    /// Shows a human-readable description of the `ErrorKind`.
    ///
    /// This is similar to `impl Display for Error`, but doesn't require first converting to Error.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// Only derive debug in tests, to make sure it
// doesn't accidentally get printed.
#[cfg_attr(test, derive(Debug))]
pub enum ErrorData<C> {
    Simple(ErrorKind),
    SimpleMessage(&'static SimpleMessage),
    Custom(C),
}

// `#[repr(align(4))]` is probably redundant, it should have that value or
// higher already. We include it just because repr_bitpacked.rs's encoding
// requires an alignment >= 4 (note that `#[repr(align)]` will not reduce the
// alignment required by the struct, only increase it).
//
// If we add more variants to ErrorData, this can be increased to 8, but it
// should probably be behind `#[cfg_attr(target_pointer_width = "64", ...)]` or
// whatever cfg we're using to enable the `repr_bitpacked` code, since only the
// that version needs the alignment, and 8 is higher than the alignment we'll
// have on 32 bit platforms.
//
// (For the sake of being explicit: the alignment requirement here only matters
// if `error/repr_bitpacked.rs` is in use — for the unpacked repr it doesn't
// matter at all)
#[repr(align(4))]
#[derive(Debug)]
pub struct SimpleMessage {
    kind: ErrorKind,
    message: &'static str,
}

impl SimpleMessage {
    pub(crate) const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }
}

// As with `SimpleMessage`: `#[repr(align(4))]` here is just because
// repr_bitpacked's encoding requires it. In practice it almost certainly be
// already be this high or higher.
#[derive(Debug)]
#[repr(align(4))]
pub struct Custom {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}
