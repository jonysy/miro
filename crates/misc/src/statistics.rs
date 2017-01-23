use num::Float;
use std::cmp::Ordering;

/// Returns the median of an array.
///
/// # Examples
///
/// ```
/// use extensions::statistics;
///
/// let median = statistics::median(&mut [3.4, 1.2, 5.6]);
///
/// assert_eq!(median, 3.4);
/// ```
///
/// ```
/// use extensions::statistics;
///
/// let median: f32 = statistics::median(&mut [3.4, 1.2, 5.6, 4.4]);
///
/// assert_eq!(median.round(), 4.0);
/// ```
pub fn median<T>(slice: &mut [T]) -> T where T: Float {
    let len = slice.len();

    if len == 0 {
        return T::zero();
    } else if len == 1 {
        return slice[0];
    } else {
        slice.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        median_sorted(slice)
    }
}

/// Returns the median of a sorted array.
///
/// # Examples
///
/// ```
/// use extensions::statistics;
///
/// let median = statistics::median_sorted(&[1.2, 3.4, 5.6]);
///
/// assert_eq!(median, 3.4);
///
/// ```
///
/// ```
/// use extensions::statistics;
///
/// let median: f32 = statistics::median_sorted(&[1.2, 3.4, 4.4, 5.6]);
///
/// assert_eq!(median.round(), 4.0);
/// ```
pub fn median_sorted<T>(sorted_slice: &[T]) -> T where T: Float {
    let len = sorted_slice.len();

    if len == 0 {
        return T::zero();
    } else if len == 1 {
        return sorted_slice[0];
    } else {
        let half = len / 2;
        
        if half % 2 == 0 {
            let two = T::one() + T::one();
            // there are two central numbers, so find their mean.
            let n = sorted_slice[half - 1] + sorted_slice[half];
            n / two
        } else {
            sorted_slice[half]
        }
    }
}