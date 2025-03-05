// Why?
// because &'static str is no longer ConstParamTy (or ConstParamTy_)
// We need BasisElementDisplayName to be ConstParamTy_ to keep things simple and organized,
// and using UnsizedConstParamTy is infectious and annoying

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::{ConstParamTy_, UnsizedConstParamTy};
use const_panic::{PanicFmt, PanicVal};
use const_panic::fmt::ShortString;

// Define the fixed-size string struct
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct fstr<const N: usize> {
    data: [u8; N],
    len: usize,
}

// Core methods
impl<const N: usize> fstr<N> {
    /// Creates an `fstr<N>` from a `&'static str` if it fits within `N` bytes.
    /// Returns `None` if the string is too long.
    pub const fn from_str(s: &'static str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() > N {
            None
        } else {
            let mut data = [0; N];
            let mut i = 0;
            while i < bytes.len() {
                data[i] = bytes[i];
                i += 1;
            }
            Some(fstr { data, len: bytes.len() })
        }
    }

    /// Converts the `fstr<N>` back to a `&str`.
    pub const fn as_str(&self) -> &str {
        // Safety: `from_str` only succeeds if the entire string fits within N bytes.
        // Since the input is a valid UTF-8 `&'static str`, the bytes in `data` up to `len`
        // are guaranteed to be valid UTF-8.
        let slice = unsafe { std::slice::from_raw_parts(self.data.as_ptr(), self.len) };
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
}

// Debug trait to display the string content
impl<const N: usize> fmt::Debug for fstr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

// Display trait to format the string content
impl<const N: usize> fmt::Display for fstr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

// Hash trait based on the string content
impl<const N: usize> Hash for fstr<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

// Implement ConstParamTy_ for use in const generics
impl<const N: usize> UnsizedConstParamTy for fstr<N> {}
impl<const N: usize> ConstParamTy_ for fstr<N> {}


impl<const N: usize> PanicFmt for fstr<N> {
    type This = Self;
    type Kind = const_panic::IsCustomType;
    const PV_COUNT: usize = 1;
}

impl<const N: usize> fstr<N> {
    pub const fn to_panicvals(self, _: const_panic::FmtArg) -> [PanicVal<'static>; 1] {
        [PanicVal::write_short_str(ShortString::new(self.as_str()))]
    }
}