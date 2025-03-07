use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Norm
/// Norm for flat aspect.
pub trait Norm {
    fn norm(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static norm: NormPrefixOrPostfix = NormPrefixOrPostfix;
pub struct NormPrefixOrPostfix;
impl<A: Norm> std::ops::Div<A> for NormPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.norm()
    }
}
include!("./impls/norm.rs");
