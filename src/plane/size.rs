use std::fmt;
use std::ops::Mul;

#[derive(Debug)]
pub struct Size<I = f64> {
    /// The width.
    pub width: I,
    /// The height.
    pub height: I,
}

impl<I> Size<I> {
    pub fn area(&self) -> I::Output where I: Copy + Mul {
        self.width * self.height
    }
}

impl<I> Clone for Size<I> where I: Clone {
    fn clone(&self) -> Self {
        Size { width: self.width.clone(), height: self.height.clone() }
    }
}

impl<I> Copy for Size<I> where I: Copy { }

impl<I> Eq for Size<I> where I: Eq { }

impl<I> PartialEq for Size<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<I> fmt::Display for Size<I> where I: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl<I> From<(I, I)> for Size<I> {
    fn from((width, height): (I, I)) -> Self {
        Size { width, height }
    }
}

impl<I> From<[I; 2]> for Size<I> {
    fn from([width, height]: [I; 2]) -> Self {
        Size { width, height }
    }
}