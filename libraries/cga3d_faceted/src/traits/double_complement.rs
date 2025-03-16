use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DoubleComplement
/// Repeatedly taking a Complement will eventually return the original object. In geometric algebras with an even number of dimensions, double_complement(x) = right_complement(right_complement(x)) = left_complement(left_complement(x)). In geometric algebras with an odd number of dimensions, double_complement(x) = complement(complement(x)). In all cases, x = double_complement(double_complement(x)).
pub trait DoubleComplement {
    fn double_complement(self) -> Self;
}
include!("./impls/double_complement.rs");
