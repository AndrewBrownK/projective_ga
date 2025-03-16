use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNormSquared
/// Intermediate result to RoundBulkNorm.
pub trait RoundBulkNormSquared {
    fn round_bulk_norm_squared(self) -> Scalar;
}
include!("./impls/round_bulk_norm_squared.rs");
