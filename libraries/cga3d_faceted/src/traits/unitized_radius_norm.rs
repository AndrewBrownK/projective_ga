use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRadiusNorm
/// Unitized radius of an object.
pub trait UnitizedRadiusNorm {
    fn unitized_radius_norm(self) -> f32;
}
include!("./impls/unitized_radius_norm.rs");
