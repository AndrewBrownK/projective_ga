use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNorm
/// Unitized Norm for round aspect.
pub trait UnitizedRoundNorm {
    fn unitized_round_norm(self) -> f32;
}
include!("./impls/unitized_round_norm.rs");
