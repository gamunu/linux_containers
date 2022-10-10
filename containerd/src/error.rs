//Error handling
pub type Result<T> = std::result::Result<T, Error>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub enum Error {
    Unknown(String),
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    FailedPrecondition(String),
    Unavailable(String),
    NotImplemented(String),
}

impl Error {
    fn description(&self) -> &str {
        match &*self {
            Error::Unknown(string) => "unknown",
            Error::InvalidArgument(string) => "invalid argument",
            Error::NotFound(string) => "not found",
            Error::AlreadyExists(string) => "already exists",
            Error::FailedPrecondition(string) => "failed precondition",
            Error::Unavailable(string) => "unavailable",
            Error::NotImplemented(string) => "not implemented",
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.description(), self)
    }
}
