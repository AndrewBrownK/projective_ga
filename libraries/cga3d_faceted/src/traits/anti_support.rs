use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiSupport
/// The anti-support is the anti-vector furthest from the origin that encloses the object.
pub trait AntiSupport {
    type Output;
    fn anti_support(self) -> Self::Output;
}
include!("./impls/anti_support.rs");
