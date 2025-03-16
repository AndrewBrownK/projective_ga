use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Carrier
/// The Carrier of a round object is the lowest dimensional flat object that contains it.
pub trait Carrier {
    type Output;
    fn carrier(self) -> Self::Output;
}
include!("./impls/carrier.rs");
