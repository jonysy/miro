//! Functionality for partial ordering and comparison.

/// Partially compare and return the minimum of two floating-point values.
///
/// # Examples
///
/// ```
/// use extensions::parcmp;
///
/// assert_eq!(1.0, parcmp::min(1.0, 2.0));
/// assert_eq!(2.0, parcmp::min(2.0, 2.0));
/// ```
#[inline]
pub fn min<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 <= v2 { v1 } else { v2 }
}

/// Partially compare and return the maximum of two floating-point values.
///
/// # Examples
///
/// ```
/// use extensions::parcmp;
///
/// assert_eq!(2.0, parcmp::max(1.0, 2.0));
/// assert_eq!(2.0, parcmp::max(2.0, 2.0));
/// ```
#[inline]
pub fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1 >= v2 { v1 } else { v2 }
}