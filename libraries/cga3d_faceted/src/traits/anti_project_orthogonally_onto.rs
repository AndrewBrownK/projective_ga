use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiProjectOrthogonallyOnto
/// Typically involves bringing a higher dimensional object to a lower dimensional object.
pub trait AntiProjectOrthogonallyOnto<T> {
    type Output;
    fn anti_project_orthogonally_onto(self, other: T) -> Self::Output;
}
include!("./impls/anti_project_orthogonally_onto.rs");
