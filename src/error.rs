//! # Example
//!
//! The following example shows simple command line parsing for an application that
//! requires a number of iterations between zero *(0)* and ten *(10)* to be specified,
//! accepts an optional log file name and responds to the help flag.

use std::{error,io};
use std::fmt::{self, Display, Formatter};

use operation::Operation;

/// An implementation of `Error` which may or may not include a cause.
#[derive(Debug)]
pub struct Error {
    cause: Option<io::Error>,
    message: String
}

impl Error {
    // Constructors
    /// Creates a new `Error` with the provided `Operation` on the given path.
    pub fn new(operation: &Operation, path: &str) -> Self {
        Error { message: Self::message(operation, path), cause: None }
    }

    /// Creates a new `Error` with the provided `Operation` on the given path
    /// and specifies the upstream cause of the error.
    pub fn with_cause(operation: &Operation, path: &str, cause: io::Error) -> Self {
        Error { message: Self::message(operation, path), cause: Some(cause) }
    }

    // Public instance methods
    /// Returns a reference to the upstream cause of this error if one exists.
    pub fn cause(&self) -> Option<&io::Error> {
        self.cause.as_ref()
    }

    fn message(operation: &Operation, path: &str) -> String {
        format!("Unable to {} {}", operation, path)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
