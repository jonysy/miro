use std::{error, fmt};
use super::Category;

#[derive(Debug)]
pub struct Error {
    category: Category,
    error: Box<error::Error + Send + Sync>,
}

impl Error {

	/// Creates a new error.
	///
	/// # Arguments
    ///
    /// * `category` - A known kind of error.
    /// * `error` - An arbitrary error payload which will be contained in `Error`.
	pub fn new<I, E>(category: I, error: E) -> Error 
		where I: Into<Category>, 
			  E: Into<Box<error::Error + Send + Sync>> {

		Error { category: category.into(), error: error.into() }
	}
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl error::Error for Error {

    fn description(&self) -> &str {  
    	use super::Category::*;
    	use super::{MotionCategory, TrackingCategory};

    	match self.category {
    		Motion(MotionCategory::Generic) => 
    			"An error occurred while analyzing information related to motion.",

    		Tracking(TrackingCategory::Generic) =>
    			"An error occurred during tracking.",

    		_ => "Something unexpected went wrong!",
    	}
    }
    
    fn cause(&self) -> Option<&error::Error> {
        Some(&*self.error)
    }
}