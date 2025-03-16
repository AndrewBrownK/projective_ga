use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundNorm
/// Norm for Round aspect.
pub trait RoundNorm {
    fn round_norm(self) -> MultiVector;
}
include!("./impls/round_norm.rs");
