use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RejectViaOriginFrom
/// Counterpart to ProjectViaOriginOnto.
pub trait RejectViaOriginFrom<T> {
    type Output;
    fn reject_via_origin_from(self, other: T) -> Self::Output;
}
include!("./impls/reject_via_origin_from.rs");
