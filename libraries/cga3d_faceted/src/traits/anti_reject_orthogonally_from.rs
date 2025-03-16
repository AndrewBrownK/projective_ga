use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiRejectOrthogonallyFrom
/// Counterpart to AntiProjectOrthogonallyOnto.
pub trait AntiRejectOrthogonallyFrom<T> {
    type Output;
    fn anti_reject_orthogonally_from(self, other: T) -> Self::Output;
}
include!("./impls/anti_reject_orthogonally_from.rs");
