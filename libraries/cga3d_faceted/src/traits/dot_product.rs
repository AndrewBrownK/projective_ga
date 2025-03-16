use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DotProduct
/// This dot product is almost exactly what you would expect from regular vector algebra. It always returns a scalar result. It is determined by the metric of the algebra, so intermediate terms might come out to zero or negative, depending on the generator squares. The "dot product" is overloaded to several different meanings depending on which community you discuss with, so if someone or something refers to the "dot product" then use care and double check the definition that is intended.
pub trait DotProduct<T> {
    fn dot_product(self, other: T) -> Scalar;
}
include!("./impls/dot_product.rs");
