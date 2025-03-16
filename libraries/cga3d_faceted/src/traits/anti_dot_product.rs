use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiDotProduct
/// This is the dual to the dot product, and always returns an AntiScalar.
pub trait AntiDotProduct<T> {
    fn anti_dot_product(self, other: T) -> AntiScalar;
}
include!("./impls/anti_dot_product.rs");
