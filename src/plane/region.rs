use num::{One, Zero};
use std::ops;
use std::ops::{Add, AddAssign};
use super::{Coordinates, Dimensions};

#[derive(Debug)]
pub struct Region<I = f64> {
    /// The position of the top left corner of the region in relation to a reference coordinates.
    pub coordinates: Coordinates<I>,
    /// The dimensions.
    pub dimensions: Dimensions<I>,
}

impl<I> Region<I> where I: Copy {
    pub fn new(x: I, y: I, width: I, height: I) -> Region<I> {
        let coordinates = Coordinates { x, y };
        let dimensions = Dimensions { width, height };
        
        Region { coordinates, dimensions }    
    }
    
    pub fn iter(&self) -> RegionIter<I> where I: Zero {
        RegionIter::new(self)
    }
    
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
    
    /// Returns the point at the top-left corner.
    pub fn top_left(&self) -> Coordinates<I> where I: Add<Output=I> {
        self.min()
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

impl<I> ops::Index<usize> for Region<I> {
    type Output = I;
    
    fn index(&self, index: usize) -> &I {
        match index {
            0 => &self.coordinates[0],
            1 => &self.coordinates[1],
            2 => &self.dimensions[0],
            3 => &self.dimensions[1],
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
}

impl<I> ops::IndexMut<usize> for Region<I> {
    fn index_mut(&mut self, index: usize) -> &mut I {
        match index {
            0 => &mut self.coordinates[0],
            1 => &mut self.coordinates[1],
            2 => &mut self.dimensions[0],
            3 => &mut self.dimensions[1],
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
}

pub struct RegionIter<'a, I> where I: 'a {
    current: Coordinates<I>,
    region: &'a Region<I>,
}

impl<'a, I> RegionIter<'a, I> where I: 'a {
    
    fn new(region: &'a Region<I>) -> Self where I: Zero {
        let current = Coordinates { x: I::zero(), y: I::zero() };
        
        RegionIter { current, region }
    }
}

impl<'a, I> Iterator for RegionIter<'a, I> where I: 'a + AddAssign + Copy + One + PartialOrd + Zero {
    type Item = (I, I);
    
    fn next(&mut self) -> Option<(I, I)> {
        if self.current[0] + self.region[0] >= self.region[2] {
            self.current.x = I::zero();
            self.current.y += I::one();
        }
        
        if self.current[1] + self.region[1] >= self.region[3] {
            return None;
        }
        
        let x_coordinate = self.current.x + self.region.coordinates.x;
        let y_coordinate = self.current.y + self.region.coordinates.y;
            
        self.current.x += I::one();
            
        Some((x_coordinate, y_coordinate))
    }
}