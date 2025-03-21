use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricAntiQuotient
/// AntiProduct of A with AntiInverse of B.
pub trait GeometricAntiQuotient<T> {
    type Output;
    fn geometric_anti_quotient(self, other: T) -> Self::Output;
}
include!("./impls/geometric_anti_quotient.rs");
