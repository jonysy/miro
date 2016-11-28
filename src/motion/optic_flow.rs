use std::error::Error;

/// [Optical flow][1] or optic flow is the pattern of apparent motion of objects, surfaces, and
/// edges in a visual scene caused by the relative motion between an
/// observer (an eye or a camera) and the scene.
///
/// [1]: https://en.wikipedia.org/wiki/Optical_flow
pub trait OpticFlow {
    
    type Err: Error;
}