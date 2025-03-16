use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ProjectOrthogonallyOnto
/// Typically involves bringing a lower dimensional object to a higher dimensional object.
pub trait ProjectOrthogonallyOnto<T> {
    type Output;
    fn project_orthogonally_onto(self, other: T) -> Self::Output;
}
include!("./impls/project_orthogonally_onto.rs");
