use std::fmt;

/// `Point` represents a point in 2-D space.
#[derive(Debug)]
pub struct Point<I = f64> {
    /// The x-coordinate.
    pub x: I,
    /// The y-coordinate.
    pub y: I,
}

impl<I> Clone for Point<I> where I: Clone {
    fn clone(&self) -> Self {
        Point { x: self.x.clone(), y: self.y.clone() }
    }
}

impl<I> Copy for Point<I> where I: Copy { }

impl<I> Eq for Point<I> where I: Eq { }

impl<I> PartialEq for Point<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<I> fmt::Display for Point<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<I> From<(I, I)> for Point<I> {
    fn from((x, y): (I, I)) -> Self {
        Point { x, y }
    }
}

impl<I> From<[I; 2]> for Point<I> {
    fn from([x, y]: [I; 2]) -> Self {
        Point { x, y }
    }
}