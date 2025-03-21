use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNormSquared
/// Intermediate result for RoundNorm.
pub trait RoundNormSquared {
    fn round_norm_squared(self) -> MultiVector;
}
include!("./impls/round_norm_squared.rs");
