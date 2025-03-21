use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulkNorm
/// BulkNorm for flat aspect.
pub trait FlatBulkNorm {
    fn flat_bulk_norm(self) -> Scalar;
}
include!("./impls/flat_bulk_norm.rs");
