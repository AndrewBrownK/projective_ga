use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ProjectViaOriginOnto
/// Central (to origin) Projection.
pub trait ProjectViaOriginOnto<T> {
    type Output;
    fn project_via_origin_onto(self, other: T) -> Self::Output;
}
include!("./impls/project_via_origin_onto.rs");
