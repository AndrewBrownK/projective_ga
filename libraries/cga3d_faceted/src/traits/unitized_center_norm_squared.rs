use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNormSquared
/// Intermediate result to UnitizedCenterNorm.
pub trait UnitizedCenterNormSquared {
    fn unitized_center_norm_squared(self) -> f32;
}
include!("./impls/unitized_center_norm_squared.rs");
