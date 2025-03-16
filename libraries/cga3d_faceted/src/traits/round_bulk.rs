use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundBulk
/// This is the aspect of a round object that characterizes the carrier's relationship with the origin.
pub trait RoundBulk {
    type Output;
    fn round_bulk(self) -> Self::Output;
}
include!("./impls/round_bulk.rs");
