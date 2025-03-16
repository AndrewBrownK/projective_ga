use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CenterNorm
/// Distance between origin and center (not yet unitized, still requires division by round weight).
pub trait CenterNorm {
    fn center_norm(self) -> Scalar;
}
include!("./impls/center_norm.rs");
