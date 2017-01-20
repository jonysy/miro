pub use self::track::Track;
mod track;

use std::error::Error;

pub trait Tracker {
    
    type Err: Error;
}