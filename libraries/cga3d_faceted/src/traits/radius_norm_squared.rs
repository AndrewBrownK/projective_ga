use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RadiusNormSquared
/// Intermediate result to RadiusNorm.
pub trait RadiusNormSquared {
    fn radius_norm_squared(self) -> Scalar;
}
include!("./impls/radius_norm_squared.rs");
