use std::marker::{ConstParamTy_, UnsizedConstParamTy};

/// Option does implement StructuralPartialEq, but
/// Option does not implement ConstParamTy.
/// So we should probably PR rust to make Option implement ContParamTy, but
/// until then, here we go.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum ConstOption<T> {
    None,
    Some(T),
}
impl<T: ConstParamTy_> ConstParamTy_ for ConstOption<T> {}
impl<T: UnsizedConstParamTy> UnsizedConstParamTy for ConstOption<T> {}
impl<T: Copy> ConstOption<T> {
    pub const fn from_option(opt: Option<T>) -> Self {
        match opt {
            None => ConstOption::None,
            Some(t) => ConstOption::Some(t),
        }
    }

    pub const fn into_option(self) -> Option<T> {
        match self {
            ConstOption::None => None,
            ConstOption::Some(t) => Some(t),
        }
    }

    pub const fn is_none(&self) -> bool {
        match self {
            ConstOption::None => true,
            ConstOption::Some(_) => false,
        }
    }

    pub const fn is_some(&self) -> bool {
        match self {
            ConstOption::None => false,
            ConstOption::Some(_) => true,
        }
    }

    pub const fn expect(self, err: &'static str) -> T {
        self.into_option().expect(err)
    }
}