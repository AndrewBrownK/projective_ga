use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiWedge
/// The AntiWedge product is the dual operation to the Wedge product, that depends on a specified AntiScalar. It combines BasisElements by which parts are missing, instead of which parts are present. For example, with an AntiScalar of e1234, anti_wedge(e423, e321) = e23. This behaves something like an intersection of the subscripts in the BasisElements.
pub trait AntiWedge<T> {
    type Output;
    fn anti_wedge(self, other: T) -> Self::Output;
}
include!("./impls/anti_wedge.rs");
