use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNormSquared
/// Intermediate result for RoundWeight.
pub trait RoundWeightNormSquared {
    fn round_weight_norm_squared(self) -> AntiScalar;
}
include!("./impls/round_weight_norm_squared.rs");
