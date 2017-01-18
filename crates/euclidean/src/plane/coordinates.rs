use std::{fmt, ops};

/// `Coordinates` represents a point in 2-D space.
#[derive(Debug)]
pub struct Coordinates<I = f64> {
    /// The x-coordinate.
    pub x: I,
    /// The y-coordinate.
    pub y: I,
}

impl<I> Clone for Coordinates<I> where I: Clone {
    fn clone(&self) -> Self {
        Coordinates { x: self.x.clone(), y: self.y.clone() }
    }
}

impl<I> Copy for Coordinates<I> where I: Copy { }

impl<I> Eq for Coordinates<I> where I: Eq { }

impl<I> PartialEq for Coordinates<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<I> fmt::Display for Coordinates<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<I> From<(I, I)> for Coordinates<I> {
    fn from((x, y): (I, I)) -> Self {
        Coordinates { x: x, y: y }
    }
}

impl<I> From<[I; 2]> for Coordinates<I> {
    fn from([x, y]: [I; 2]) -> Self {
        Coordinates { x: x, y: y }
    }
}

impl<I> ops::Index<usize> for Coordinates<I> {
    type Output = I;
    
    fn index(&self, index: usize) -> &I {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
}

impl<I> ops::IndexMut<usize> for Coordinates<I> {
    fn index_mut(&mut self, index: usize) -> &mut I {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
}