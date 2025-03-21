use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNormSquared
/// Intermediate result for FlatBulkNorm.
pub trait FlatBulkNormSquared {
    fn flat_bulk_norm_squared(self) -> Scalar;
}
include!("./impls/flat_bulk_norm_squared.rs");
