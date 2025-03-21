use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulkNorm
/// Bulk Norm for round aspect.
pub trait RoundBulkNorm {
    fn round_bulk_norm(self) -> Scalar;
}
include!("./impls/round_bulk_norm.rs");
