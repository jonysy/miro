use std::fmt;
use std::ops::Mul;

#[derive(Debug)]
pub struct Dimensions<I = f64> {
    /// The width.
    pub width: I,
    /// The height.
    pub height: I,
}

impl<I> Dimensions<I> {
    pub fn area(&self) -> I::Output where I: Copy + Mul {
        self.width * self.height
    }
}

impl<I> Clone for Dimensions<I> where I: Clone {
    fn clone(&self) -> Self {
        Dimensions { width: self.width.clone(), height: self.height.clone() }
    }
}

impl<I> Copy for Dimensions<I> where I: Copy { }

impl<I> Eq for Dimensions<I> where I: Eq { }

impl<I> PartialEq for Dimensions<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<I> fmt::Display for Dimensions<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl<I> From<(I, I)> for Dimensions<I> {
    fn from((width, height): (I, I)) -> Self {
        Dimensions { width, height }
    }
}

impl<I> From<[I; 2]> for Dimensions<I> {
    fn from([width, height]: [I; 2]) -> Self {
        Dimensions { width, height }
    }
}