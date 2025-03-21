use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNormSquared
/// Intermediate result to FlatWeightNorm.
pub trait FlatWeightNormSquared {
    fn flat_weight_norm_squared(self) -> AntiScalar;
}
include!("./impls/flat_weight_norm_squared.rs");
