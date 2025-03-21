use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatNorm
/// Norm for flat aspect.
pub trait FlatNorm {
    fn flat_norm(self) -> MultiVector;
}
include!("./impls/flat_norm.rs");
