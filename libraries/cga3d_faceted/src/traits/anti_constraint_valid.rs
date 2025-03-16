use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiConstraintValid
/// Implementors of this trait cannot violate the anti geometric constraint. They always represent valid geometry. This trait does not exist to perform any calculation, it just exists to serve as contrasting information side-by-side with AntiConstraintViolation. See also AntiConstraintViolation and AntiFix.
pub trait AntiConstraintValid {
    fn anti_constraint_valid(self) -> Self;
}
include!("./impls/anti_constraint_valid.rs");
