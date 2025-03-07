use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Carrier
/// The Carrier of a round object is the lowest dimensional flat object that contains it.
pub trait Carrier {
    type Output;
    fn carrier(self) -> Self::Output;
}
#[allow(non_upper_case_globals, dead_code)]
pub static carrier: CarrierPrefixOrPostfix = CarrierPrefixOrPostfix;
pub struct CarrierPrefixOrPostfix;
impl<A: Carrier> std::ops::Div<A> for CarrierPrefixOrPostfix {
    type Output = <A as Carrier>::Output;
    fn div(self, rhs: A) -> Self::Output {
        rhs.carrier()
    }
}
include!("./impls/carrier.rs");
