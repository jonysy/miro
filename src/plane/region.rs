use std::ops::Add;
use super::{Dimensions, Point};

#[derive(Debug)]
pub struct Region<I = f64> {
    /// The position of the top left corner of the region in relation to a reference origin.
    pub origin: Point<I>,
    /// The dimensions.
    pub dimensions: Dimensions<I>,
}

impl<I> Region<I> where I: Copy {

    pub fn min_x(&self) -> I {
        self.origin.x
    }
    
    pub fn min_y(&self) -> I {
        self.origin.y
    }
    
    pub fn min(&self) -> Point<I> {
        Point { x: self.min_x(), y: self.min_y() }
    }
    
    pub fn max_x(&self) -> I where I: Add<Output=I> {
        self.origin.x + self.dimensions.width
    }
    
    pub fn max_y(&self) -> I where I: Add<Output=I> {
        self.origin.y + self.dimensions.height
    }
    
    /// Returns the maximum point within the bounds of the region.
    pub fn max(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.max_x(), y: self.max_y() }  
    }
    
    /// Returns the point at the top-right corner.
    pub fn top_right(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.max_x(), y: self.min_y() }
    }
    
    /// Returns the point at the bottom-left corner.
    pub fn bottom_left(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.min_x(), y: self.max_y() }
    }
    
    /// Returns the point at the bottom-right corner.
    pub fn bottom_right(&self) -> Point<I> where I: Add<Output=I> {
        self.max()
    }
}

impl<I> Clone for Region<I> where I: Clone {
    fn clone(&self) -> Region<I> {
        Region {
            origin: self.origin.clone(),
            dimensions: self.dimensions.clone()
        }
    }
}

impl<I> Copy for Region<I> where I: Copy { }

impl<I> From<(I, I, I, I)> for Region<I> {
    fn from((x, y, width, height): (I, I, I, I)) -> Region<I> {
        Region {
            origin: Point { x, y},
            dimensions: Dimensions { width, height }
        }
    }
}

impl<I> From<[I; 4]> for Region<I> {
    fn from([x, y, width, height]: [I; 4]) -> Region<I> {
        Region {
            origin: Point { x, y},
            dimensions: Dimensions { width, height }
        }
    }
}

impl<I> Into<(I, I, I, I)> for Region<I> {
    fn into(self) -> (I, I, I, I) {
        (self.origin.x, self.origin.y, self.dimensions.width, self.dimensions.height)
    }
}

impl<I> Into<[I; 4]> for Region<I> {
    fn into(self) -> [I; 4] {
        [self.origin.x, self.origin.y, self.dimensions.width, self.dimensions.height]
    }
}

impl<I> PartialEq for Region<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.dimensions == other.dimensions
    }
}

impl<I> Eq for Region<I> where I: Eq { }