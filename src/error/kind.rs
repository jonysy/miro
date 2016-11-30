/// A list of error categories.
#[derive(Debug)]
pub enum Kind {    
    Motion,
    Tracking,
    
    #[doc(hidden)]
    _NonExhaustive,
}