use num::Float;
use std::iter::Iterator;
use std::ops::AddAssign;

/// x <= max
#[derive(Clone, Copy, Debug)]
pub struct RangeInc<T>(pub T, pub T);

impl<T> Iterator for RangeInc<T> where T: AddAssign + Float {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 > self.1 {
            return None;
        }
        
        let ret = self.0;
        
        self.0 += T::one();
        
        Some(ret)
    }
}

#[test]
fn test_range_inc() {
    let mut it = RangeInc(3.0, 5.6);
    
    for x in (3...5) {
        assert_eq!(x as f32, it.next().unwrap());
    }
    
    assert_eq!(None, it.next());
}

#[test]
fn test_range_inc2() {
    let mut it = RangeInc(3.6, 5.6);
    
    for x in (3...5) {
        assert_eq!(x as f32 + 0.6, it.next().unwrap());
    }
    
    assert_eq!(None, it.next());
}