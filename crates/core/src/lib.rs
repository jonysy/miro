pub mod error;
pub use error::{Error, Kind as ErrorKind};
pub use never::Never;
pub use result::Result;

mod never;
mod result;