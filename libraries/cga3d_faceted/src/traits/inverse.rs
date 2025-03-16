use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Inverse
/// The inverse with respect to geometric product. Inverse(x) = x^-1.
pub trait Inverse {
    fn inverse(self) -> Self;
}
include!("./impls/inverse.rs");
