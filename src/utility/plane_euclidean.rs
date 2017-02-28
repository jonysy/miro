use euclidean::Point2D;
use float::{Finite, FloatGuard};

pub type Point<T> = Point2D<FloatGuard<T, Finite>>;

pub type OptPoint<T> = Option<Point<T>>;