pub use self::kind::Kind::{self, Motion as MotionErr, Tracking as TrackingErr};
mod kind;

use std::{error, fmt};

/// The error type.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    error: Box<error::Error>,
}

impl Error {
    /// Creates a new error.
    ///
    /// # Arguments
    ///
    /// * `kind` - A known kind of error.
    /// * `error` - An arbitrary error payload.
    pub fn new<E>(kind: Kind, e: E) -> Error
        where E: Into<Box<error::Error>> {
            
        Error { kind: kind, error: e.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {         
        match self.kind {
            MotionErr => "An error occured while analysing information related to motion.",
            TrackingErr => "An error occured during tracking.",
            _ => "Something unexpected went wrong!",
        }
    }
    
    fn cause(&self) -> Option<&error::Error> {
        Some(&*self.error)
    }
}