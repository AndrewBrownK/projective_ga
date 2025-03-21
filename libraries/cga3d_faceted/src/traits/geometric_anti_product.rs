use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// GeometricAntiProduct
/// The GeometricAntiProduct or sometimes called AntiProduct is the dual to the GeometricProduct. It depends on a specified AntiScalar. Anti-Multiplying uniform grade geometry may result in mixed grade anti-products, bottoming out at the Scalar. See also AntiSandwich.
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}
include!("./impls/geometric_anti_product.rs");
