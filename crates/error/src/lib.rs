//! Types modeling the success and failure of error-prone computations.
//!
//! [RFC 1216]: https://github.com/rust-lang/rust/issues/35121

pub use category::{Category, MotionCategory, TrackingCategory};
pub use concrete::Error;
pub use never::Never;
pub use result::Result;

mod category;
mod concrete;
mod never;
mod result;