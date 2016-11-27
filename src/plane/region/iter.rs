use num::{One, Zero};
use std::ops::AddAssign;
use plane::{Coordinates, Region};

pub struct Iter<'a, I> where I: 'a {
    current: Coordinates<I>,
    region: &'a Region<I>,
}

impl<'a, I> Iter<'a, I> where I: 'a {
    
    pub fn new(region: &'a Region<I>) -> Self where I: Zero {
        let current = Coordinates { x: I::zero(), y: I::zero() };
        
        Iter { current, region }
    }
}

impl<'a, I> Iterator for Iter<'a, I> where I: 'a + AddAssign + Copy + One + PartialOrd + Zero {
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