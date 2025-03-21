use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeightNorm
/// Weight Norm for flat aspect.
pub trait FlatWeightNorm {
    fn flat_weight_norm(self) -> AntiScalar;
}
include!("./impls/flat_weight_norm.rs");
