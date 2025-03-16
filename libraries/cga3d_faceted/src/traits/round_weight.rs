use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// RoundWeight
/// This is the aspect of a round object that characterizes the carrier's relationship with the horizon.
pub trait RoundWeight {
    type Output;
    fn round_weight(self) -> Self::Output;
}
include!("./impls/round_weight.rs");
