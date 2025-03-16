use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// WeightContraction
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait WeightContraction<T> {
    type Output;
    fn weight_contraction(self, other: T) -> Self::Output;
}
include!("./impls/weight_contraction.rs");
