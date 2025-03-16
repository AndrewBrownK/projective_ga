use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatBulk
/// This characterizes the flat aspect's relationship with the origin.
pub trait FlatBulk {
    type Output;
    fn flat_bulk(self) -> Self::Output;
}
include!("./impls/flat_bulk.rs");
