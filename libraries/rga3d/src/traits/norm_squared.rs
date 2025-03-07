use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// NormSquared
/// Intermediate result to FlatNorm.
pub trait NormSquared {
    fn norm_squared(self) -> AntiScalar;
}
#[allow(non_upper_case_globals, dead_code)]
pub static norm_squared: NormSquaredPrefixOrPostfix = NormSquaredPrefixOrPostfix;
pub struct NormSquaredPrefixOrPostfix;
impl<A: NormSquared> std::ops::Div<A> for NormSquaredPrefixOrPostfix {
    type Output = AntiScalar;
    fn div(self, rhs: A) -> Self::Output {
        rhs.norm_squared()
    }
}
include!("./impls/norm_squared.rs");
