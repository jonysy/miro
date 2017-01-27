//! A modern, (soon to be) GPU-accelerated computer vision library providing state-of-the-art 
//! implementations and performance.
//!
//! ## Philosophy
//! 
//! * __Application Driven__:
//! For a more research-driven approach, have a look at [illusions][illusions-github] 
//! and [mirage][mirage-github].
//! * __Well-Written Documentation__:
//! Documentation includes references, pseudo-code, brief descriptions illustrating the concept of the algorithms and data-structures, ..
//! * __Modular Design__:
//! The crate itself is made up of μcrates. Algorithms used by other algorithms can easily be replaced with an alternative.
//!
//! [illusions-github]: https://github.com/lychee-eng/illusions
//! [mirage-github]: https://github.com/lychee-eng/mirage

pub extern crate miro_classification as classification;
pub extern crate miro_core as core;
pub extern crate miro_euclidean as euclidean;
pub extern crate miro_extn as extn;
pub extern crate miro_misc as misc;
pub extern crate miro_motion as motion;
pub extern crate miro_tracking as tracking;