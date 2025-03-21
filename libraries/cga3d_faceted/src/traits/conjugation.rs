use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Conjugation
/// This composes the reverse and grade involution (automorphism).
pub trait Conjugation {
    fn conjugation(self) -> Self;
}
include!("./impls/conjugation.rs");
