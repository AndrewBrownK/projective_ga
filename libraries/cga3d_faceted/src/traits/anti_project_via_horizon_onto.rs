use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiProjectViaHorizonOnto
/// Outward (to horizon) AntiProjection.
pub trait AntiProjectViaHorizonOnto<T> {
    type Output;
    fn anti_project_via_horizon_onto(self, other: T) -> Self::Output;
}
include!("./impls/anti_project_via_horizon_onto.rs");
