use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNormSquared
/// Intermediate result to UnitizedRadiusNorm.
pub trait UnitizedRadiusNormSquared {
    fn unitized_radius_norm_squared(self) -> f32;
}
include!("./impls/unitized_radius_norm_squared.rs");
