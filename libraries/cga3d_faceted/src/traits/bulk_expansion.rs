use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// BulkExpansion
/// This is an interior product (contrast with inner product and exterior product). The interior products are derived by Wedging (or AntiWedging) one object with the Dual (or AntiDual) of another object.
pub trait BulkExpansion<T> {
    type Output;
    fn bulk_expansion(self, other: T) -> Self::Output;
}
include!("./impls/bulk_expansion.rs");
