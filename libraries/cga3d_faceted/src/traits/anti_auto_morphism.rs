use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiAutoMorphism
/// Negate every BasisElement with an odd AntiGrade.
pub trait AntiAutoMorphism {
    fn anti_auto_morphism(self) -> Self;
}
include!("./impls/anti_auto_morphism.rs");
