use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeightNorm
/// Weight Norm for round aspect.
pub trait RoundWeightNorm {
    fn round_weight_norm(self) -> AntiScalar;
}
include!("./impls/round_weight_norm.rs");
