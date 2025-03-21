use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ConformalConjugate
/// The conformal conjugate negates the flat elements of an object, and is useful in calculating the center norm of the object.
pub trait ConformalConjugate {
    fn conformal_conjugate(self) -> Self;
}
include!("./impls/conformal_conjugate.rs");
