use crate::data::*;
use crate::simd::*;

/// One
/// The scalar element set to one, and all other elements set to zero.
pub trait One {
    fn one() -> Self;
}
#[allow(non_camel_case_types)]
pub struct one;
include!("./impls/one.rs");
