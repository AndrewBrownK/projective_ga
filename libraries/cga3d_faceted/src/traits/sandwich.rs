use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Sandwich
/// The so-called "sandwich product" squeezes some factor A between another factor B and the reversal of B. This is frequently used to represent geometric transformations, for example reflecting across a plane or rotating around a line.
pub trait Sandwich<T> {
    type Output;
    fn sandwich(self, other: T) -> Self::Output;
}
include!("./impls/sandwich.rs");
