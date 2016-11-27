use std::ops::Add;
use super::{Coordinates, Dimensions};

#[derive(Debug)]
pub struct Region<I = f64> {
    /// The position of the top left corner of the region in relation to a reference coordinates.
    pub coordinates: Coordinates<I>,
    /// The dimensions.
    pub dimensions: Dimensions<I>,
}

impl<I> Region<I> where I: Copy {

    pub fn min_x(&self) -> I {
        self.coordinates.x
    }
    
    pub fn min_y(&self) -> I {
        self.coordinates.y
    }
    
    pub fn min(&self) -> Coordinates<I> {
        Coordinates { x: self.min_x(), y: self.min_y() }
    }
    
    pub fn max_x(&self) -> I where I: Add<Output=I> {
        self.coordinates.x + self.dimensions.width
    }
    
    pub fn max_y(&self) -> I where I: Add<Output=I> {
        self.coordinates.y + self.dimensions.height
    }
    
    /// Returns the maximum point within the bounds of the region.
    pub fn max(&self) -> Coordinates<I> where I: Add<Output=I> {
        Coordinates { x: self.max_x(), y: self.max_y() }  
    }
    
    /// Returns the point at the top-right corner.
    pub fn top_right(&self) -> Coordinates<I> where I: Add<Output=I> {
        Coordinates { x: self.max_x(), y: self.min_y() }
    }
    
    /// Returns the point at the bottom-left corner.
    pub fn bottom_left(&self) -> Coordinates<I> where I: Add<Output=I> {
        Coordinates { x: self.min_x(), y: self.max_y() }
    }
    
    /// Returns the point at the bottom-right corner.
    pub fn bottom_right(&self) -> Coordinates<I> where I: Add<Output=I> {
        self.max()
    }
}

impl<I> Clone for Region<I> where I: Clone {
    fn clone(&self) -> Region<I> {
        Region {
            coordinates: self.coordinates.clone(),
            dimensions: self.dimensions.clone()
        }
    }
}

impl<I> Copy for Region<I> where I: Copy { }

impl<I> From<(I, I, I, I)> for Region<I> {
    fn from((x, y, width, height): (I, I, I, I)) -> Region<I> {
        Region {
            coordinates: Coordinates { x, y},
            dimensions: Dimensions { width, height }
        }
    }
}

impl<I> From<[I; 4]> for Region<I> {
    fn from([x, y, width, height]: [I; 4]) -> Region<I> {
        Region {
            coordinates: Coordinates { x, y},
            dimensions: Dimensions { width, height }
        }
    }
}

impl<I> Into<(I, I, I, I)> for Region<I> {
    fn into(self) -> (I, I, I, I) {
        (self.coordinates.x, self.coordinates.y, self.dimensions.width, self.dimensions.height)
    }
}

impl<I> Into<[I; 4]> for Region<I> {
    fn into(self) -> [I; 4] {
        [self.coordinates.x, self.coordinates.y, self.dimensions.width, self.dimensions.height]
    }
}

impl<I> PartialEq for Region<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates && self.dimensions == other.dimensions
    }
}

impl<I> Eq for Region<I> where I: Eq { }