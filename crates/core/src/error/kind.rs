/// A list specifying general error categories.
#[derive(Debug)]
pub enum Kind {
    Motion,
    Tracking,
    
    #[doc(hidden)]
    _NonExhaustive,
}