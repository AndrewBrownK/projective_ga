use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNormSquared
/// Intermediate result to UnitizedFlatNorm.
pub trait UnitizedFlatNormSquared {
    fn unitized_flat_norm_squared(self) -> f32;
}
include!("./impls/unitized_flat_norm_squared.rs");
