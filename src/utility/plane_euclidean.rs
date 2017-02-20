use euclidean::Point2D;
#[cfg(feature = "float_guard")]
use float::{Finite, FloatGuard};

#[cfg(feature = "float_guard")]
pub type Point<T> = Point2D<FloatGuard<T, Finite>>;

#[cfg(not(feature = "float_guard"))]
pub type Point<T> = Point2D<T>;

pub type OptPoint<T> = Option<Point<T>>;