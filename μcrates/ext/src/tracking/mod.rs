pub use self::track_fn::TrackFn;
mod track_fn;

use std::error::Error;

pub trait Tracker {
    
    type Err: Error;
}