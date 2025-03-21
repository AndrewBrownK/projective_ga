use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedFlatNorm
/// Unitized FlatNorm.
pub trait UnitizedFlatNorm {
    fn unitized_flat_norm(self) -> f32;
}
include!("./impls/unitized_flat_norm.rs");
