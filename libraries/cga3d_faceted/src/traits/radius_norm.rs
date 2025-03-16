use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RadiusNorm
/// Distance radius of a round object (not yet unitized, still requires division by round weight).
pub trait RadiusNorm {
    fn radius_norm(self) -> Scalar;
}
include!("./impls/radius_norm.rs");
