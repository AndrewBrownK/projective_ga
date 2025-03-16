use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedCenterNorm
/// Unitized distance from origin to center of round object.
pub trait UnitizedCenterNorm {
    fn unitized_center_norm(self) -> f32;
}
include!("./impls/unitized_center_norm.rs");
