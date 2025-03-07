use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CoCarrier
/// The CoCarrier of a round object is a flat object that intersects the center of the round object, and is perpendicular to the Carrier.
pub trait CoCarrier {
    type Output;
    fn co_carrier(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static co_carrier: CoCarrierPrefixOrPostfix = CoCarrierPrefixOrPostfix;
pub struct CoCarrierPrefixOrPostfix;
impl<A: CoCarrier> std::ops::Div<A> for CoCarrierPrefixOrPostfix {
    type Output = <A as CoCarrier>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.co_carrier()
    }
}
include!("./impls/co_carrier.rs");
