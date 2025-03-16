use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricQuotient
/// Product of A with Inverse of B.
pub trait GeometricQuotient<T> {
    type Output;
    fn geometric_quotient(self, other: T) -> Self::Output;
}
include!("./impls/geometric_quotient.rs");
