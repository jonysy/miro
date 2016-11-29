use std::{error, fmt};

/// `Never` represents the type of a value that can never exist. `!` will be used in place 
/// of `Never` in the near future (issue: [tracking issue for promoting `!` to a type (RFC 1216)][1]).
///
/// [1]: https://github.com/rust-lang/rust/issues/35121
#[derive(Debug)]
pub enum Never {}

impl fmt::Display for Never {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl error::Error for Never {
    fn description(&self) -> &str {         
        unreachable!()
    }
}