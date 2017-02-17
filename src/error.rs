use std::{error, fmt, result};

/// The result of an error-prone computation.
pub type Result<T = ()> = result::Result<T, Error>;

/// The error type
#[derive(Debug)]
pub struct Error {
    error: Box<error::Error + Send + Sync>,
    kind: ErrorKind,
}

/// A list specifying general error categories.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// An error occurred while analyzing information related to motion.
    Motion,
    /// An error occurred during tracking.
    Tracking,
    /// A marker variant that tells the compiler that users of this enum cannot match 
    /// it exhaustively.
    ///
    /// [Private enum variants #32770](https://github.com/rust-lang/rust/issues/32770)
    #[doc(hidden)]
    _NonExhaustive,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        use self::ErrorKind::{Motion, Tracking};

        match *self {
            Motion => "An error occurred while analyzing information related to motion.",
            Tracking => "An error occurred during tracking.",
            _ => unreachable!(),
        }
    }
}

impl Error {

    /// Creates a new error.
    ///
    /// # Arguments
    ///
    /// * `kind` - A known kind of error.
    /// * `error` - An arbitrary error payload which will be contained in `Error`.
    pub fn new<E>(e: E, kind: ErrorKind) -> Error 
        where E: Into<Box<error::Error + Send + Sync>> {

        Self::_new(e.into(), kind)
    }

    // "De-generization" technique..
    fn _new(error: Box<error::Error + Send + Sync>, kind: ErrorKind) -> Error {

        Error { error, kind }
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind.as_str())
    }
}

impl error::Error for Error {

    fn description(&self) -> &str {  
        self.error.description()
    }
    
    fn cause(&self) -> Option<&error::Error> {
        self.error.cause()
    }
}