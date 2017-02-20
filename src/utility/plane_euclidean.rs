use euclidean::Point2D;
#[cfg(feature = "pilot")]
use float::{Finite, FloatGuard};

#[cfg(feature = "pilot")]
pub type Point<T> = Point2D<FloatGuard<T, Finite>>;

#[cfg(not(feature = "pilot"))]
pub type Point<T> = Point2D<T>;

pub type OptPoint<T> = Option<Point<T>>;