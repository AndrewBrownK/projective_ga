use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiInverse
/// The inverse with respect to the geometric anti-product.
pub trait AntiInverse {
    fn anti_inverse(self) -> Self;
}
include!("./impls/anti_inverse.rs");
