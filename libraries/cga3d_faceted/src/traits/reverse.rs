use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Reverse
/// Reversal is an operation that will negate some BasisElements. This changes the "sign" or "direction" of a BasisElement. Each BasisElement is a wedge of some generator elements, for example e12 = wedge(e1, e2). The reversal of e12 is reverse(e12) = e21 = -e12 = wedge(e2, e1). When the number of generator element position swaps is odd, then the reverse negates thee sign. Otherwise, the sign stays the same. For example, reverse(e1) = e1, and reverse(e1234) = e1234 = e4321. This is a consequence of the wedge product being anti-commutative.
pub trait Reverse {
    fn reverse(self) -> Self;
}
include!("./impls/reverse.rs");
