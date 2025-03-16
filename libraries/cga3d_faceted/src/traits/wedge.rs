use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Wedge
/// The Wedge product (also known as "Exterior Product" or Grassmann's "Progressive Combinatorial Product") combines BasisElements into higher grade BasisElements. For example, wedge(e1, e2) = e12, and wedge(e1, e23) = e123. The Wedge product is anti-commutative, so wedge(a, b) = -wedge(b, a). A non-scalar element wedged with itself is zero. This behaves something like a union of the subscripts in the BasisElements.
pub trait Wedge<T> {
    type Output;
    fn wedge(self, other: T) -> Self::Output;
}
include!("./impls/wedge.rs");
