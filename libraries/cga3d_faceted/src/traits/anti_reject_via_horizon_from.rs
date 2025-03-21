use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiRejectViaHorizonFrom
/// Counterpart to AntiProjectViaHorizonOnto.
pub trait AntiRejectViaHorizonFrom<T> {
    type Output;
    fn anti_reject_via_horizon_from(self, other: T) -> Self::Output;
}
include!("./impls/anti_reject_via_horizon_from.rs");
