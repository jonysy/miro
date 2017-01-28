use std::error::Error;

pub trait Tracker {
    
    type Err: Error;
}