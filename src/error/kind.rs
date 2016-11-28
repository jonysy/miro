/// A list of error categories.
#[derive(Debug)]
pub enum Kind {    
    Motion,
    
    #[doc(hidden)]
    _NonExhaustive,
}