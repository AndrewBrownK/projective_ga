use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Unitize
/// Scale the object to have a weight norm of 1.
pub trait Unitize {
    fn unitize(self) -> Self;
}
include!("./impls/unitize.rs");
