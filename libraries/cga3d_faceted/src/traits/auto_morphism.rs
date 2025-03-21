use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AutoMorphism
/// Negate every BasisElement with an odd Grade. Also known as grade involution.
pub trait AutoMorphism {
    fn auto_morphism(self) -> Self;
}
include!("./impls/auto_morphism.rs");
