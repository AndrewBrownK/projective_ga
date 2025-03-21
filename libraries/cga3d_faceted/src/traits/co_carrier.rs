use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CoCarrier
/// The CoCarrier of a round object is a flat object that intersects the center of the round object, and is perpendicular to the Carrier.
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}
include!("./impls/co_carrier.rs");
