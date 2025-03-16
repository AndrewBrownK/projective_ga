use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// ConstraintValid
/// Implementors of this trait cannot violate the geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with ConstraintViolation. See also ConstraintViolation and Fix.
pub trait ConstraintValid {
    fn constraint_valid(self) -> Self;
}
include!("./impls/constraint_valid.rs");
