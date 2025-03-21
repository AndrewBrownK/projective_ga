use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RejectOrthogonallyFrom
/// Counterpart to ProjectOrthogonallyOnto.
pub trait RejectOrthogonallyFrom<T> {
    type Output;
    fn reject_orthogonally_from(self, other: T) -> Self::Output;
}
include!("./impls/reject_orthogonally_from.rs");
