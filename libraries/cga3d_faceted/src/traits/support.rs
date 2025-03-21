use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Support
/// The support is the point enclosed by the object closest to the origin.
pub trait Support {
    type Output;
    fn support(self) -> Self::Output;
}
include!("./impls/support.rs");
