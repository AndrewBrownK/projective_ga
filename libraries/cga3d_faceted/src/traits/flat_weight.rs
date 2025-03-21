use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatWeight
/// This characterizes the flat aspect's relationship with the horizon.
pub trait FlatWeight {
    type Output;
    fn flat_weight(self) -> Self::Output;
}
include!("./impls/flat_weight.rs");
