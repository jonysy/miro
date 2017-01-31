/// A list specifying general error categories.
#[derive(Debug)]
pub enum Category {
    /// An error occurred while analyzing information related to motion.
    Motion(MotionCategory),

    /// An error occurred during tracking.
    Tracking(TrackingCategory),
    
    /// A marker variant that tells the compiler that users of this enum cannot match 
    /// it exhaustively.
    ///
    /// [Private enum variants #32770](https://github.com/rust-lang/rust/issues/32770)
    #[doc(hidden)]
    _NonExhaustive,
}

/// Motion specific error categories.
#[derive(Debug)]
pub enum MotionCategory {
    Generic,

    /// A marker variant that tells the compiler that users of this enum cannot match 
    /// it exhaustively.
    ///
    /// [Private enum variants #32770](https://github.com/rust-lang/rust/issues/32770)
	#[doc(hidden)]
    _NonExhaustive,
}

impl From<MotionCategory> for Category {

    fn from(motion_category: MotionCategory) -> Category {

        Category::Motion(motion_category)
    }
}

/// Tracking specific error categories.
#[derive(Debug)]
pub enum TrackingCategory {
    Generic,

    /// A marker variant that tells the compiler that users of this enum cannot match 
    /// it exhaustively.
    ///
    /// [Private enum variants #32770](https://github.com/rust-lang/rust/issues/32770)
	#[doc(hidden)]
    _NonExhaustive,
}

impl From<TrackingCategory> for Category {

    fn from(tracking_category: TrackingCategory) -> Category {

        Category::Tracking(tracking_category)
    }
}