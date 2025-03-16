use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// UnitizedRoundNormSquared
/// Intermediate result to UnitizedRoundNorm.
pub trait UnitizedRoundNormSquared {
    fn unitized_round_norm_squared(self) -> f32;
}
include!("./impls/unitized_round_norm_squared.rs");
