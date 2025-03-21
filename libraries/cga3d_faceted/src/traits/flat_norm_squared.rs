use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNormSquared
/// Intermediate result to FlatNorm.
pub trait FlatNormSquared {
    fn flat_norm_squared(self) -> MultiVector;
}
include!("./impls/flat_norm_squared.rs");
