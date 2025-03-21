use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiFix
/// Automatically fix the anti geometric constraint by adjusting the bulk to comply with the weight, and then weight normalize the result.
pub trait AntiFix {
    fn anti_fix(self) -> Self;
}
include!("./impls/anti_fix.rs");
