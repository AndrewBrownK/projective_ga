use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Fix
/// Automatically fix the geometric constraint by adjusting the weight to comply with the bulk, and then bulk normalize the result.
pub trait Fix {
    fn fix(self) -> Self;
}
include!("./impls/fix.rs");
