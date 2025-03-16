use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CenterNormSquared
/// Intermediate result to CenterNorm.
pub trait CenterNormSquared {
    fn center_norm_squared(self) -> Scalar;
}
include!("./impls/center_norm_squared.rs");
