use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// FlatOrigin
#[repr(C)]
#[derive(Clone, Copy)]
pub union FlatOrigin {
    groups: FlatOriginGroups,
    /// e45, 0, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct FlatOriginGroups {
    /// e45
    g0: f32,
}
impl FlatOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e45: f32) -> Self {
        Self { elements: [e45, 0.0, 0.0, 0.0] }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: FlatOriginGroups { g0 } }
    }
    #[inline(always)]
    pub fn group0(&self) -> f32 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g0 }
    }
}
const FLAT_ORIGIN_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for FlatOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[FLAT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for FlatOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[FLAT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<FlatOrigin> for [f32; 1] {
    fn from(vector: FlatOrigin) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for FlatOrigin {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for FlatOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("FlatOrigin").field("e45", &self[0]).finish()
    }
}

impl FlatOrigin {
    pub const LEN: usize = 1;
}

impl nearly::NearlyEqEps<FlatOrigin, f32, f32> for FlatOrigin {
    fn nearly_eq_eps(&self, other: &FlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_ne_eps(a, b, eps) {
                return false;
            }
            i += 1;
        }
        true
    }
}
impl nearly::NearlyEqUlps<FlatOrigin, f32, f32> for FlatOrigin {
    fn nearly_eq_ulps(&self, other: &FlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_ne_ulps(a, b, ulps) {
                return false;
            }
            i += 1;
        }
        true
    }
}
impl nearly::NearlyEqTol<FlatOrigin, f32, f32> for FlatOrigin {}
impl nearly::NearlyEq<FlatOrigin, f32, f32> for FlatOrigin {}
impl nearly::NearlyOrdUlps<FlatOrigin, f32, f32> for FlatOrigin {
    fn nearly_lt_ulps(&self, other: &FlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }

    fn nearly_gt_ulps(&self, other: &FlatOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqUlps::nearly_eq_ulps(a, b, ulps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }
}
impl nearly::NearlyOrdEps<FlatOrigin, f32, f32> for FlatOrigin {
    fn nearly_lt_eps(&self, other: &FlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a < b {
                // Nearly equal until less-than wins
                return true;
            } else {
                // else greater-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }

    fn nearly_gt_eps(&self, other: &FlatOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
        let mut i = 0;
        while i < Self::LEN {
            let a = &self[i];
            let b = &other[i];
            if nearly::NearlyEqEps::nearly_eq_eps(a, b, eps) {
                // Too close, compare next element
                i += 1;
                continue;
            }
            if a > b {
                // Nearly equal until greater-than wins
                return true;
            } else {
                // else less-than wins
                return false;
            }
        }
        // Nearly equal the whole way
        false
    }
}
impl nearly::NearlyOrdTol<FlatOrigin, f32, f32> for FlatOrigin {}
impl nearly::NearlyOrd<FlatOrigin, f32, f32> for FlatOrigin {}

impl FlatOrigin {
    pub fn clamp_zeros(mut self, tolerance: nearly::Tolerance<f32>) -> Self {
        for i in 0..Self::LEN {
            let f = self[i];
            if nearly::nearly!(0.0 == f, tol = tolerance) {
                self[i] = 0.0;
            }
        }
        self
    }
}

impl PartialOrd for FlatOrigin {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return Some(result),
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}
impl Ord for FlatOrigin {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            match a.cmp(&b) {
                std::cmp::Ordering::Equal => continue,
                result => return result,
            }
        }
        std::cmp::Ordering::Equal
    }
}
impl PartialEq for FlatOrigin {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..Self::LEN {
            let a = float_ord::FloatOrd(self[i]);
            let b = float_ord::FloatOrd(other[i]);
            if a != b {
                return false;
            }
        }
        true
    }
}
impl Eq for FlatOrigin {}
impl std::hash::Hash for FlatOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for FlatOrigin {}
unsafe impl bytemuck::Pod for FlatOrigin {}
impl encase::ShaderType for FlatOrigin {
    type ExtraMetadata = <FlatOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <FlatOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <FlatOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <FlatOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <FlatOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for FlatOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FlatOrigin", 1)?;
        state.serialize_field("e45", &self[crate::elements::e45])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for FlatOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum FlatOriginField {
            e45,
        }
        struct FlatOriginVisitor;
        impl<'de> Visitor<'de> for FlatOriginVisitor {
            type Value = FlatOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlatOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<FlatOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e45 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        FlatOriginField::e45 => {
                            if e45.is_some() {
                                return Err(serde::de::Error::duplicate_field("e45"));
                            }
                            e45 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = FlatOrigin::from([0.0; 1]);
                result[crate::elements::e45] = e45.ok_or_else(|| serde::de::Error::missing_field("e45"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e45"];
        deserializer.deserialize_struct("FlatOrigin", FIELDS, FlatOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e45> for FlatOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e45) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e45> for FlatOrigin {
    fn index_mut(&mut self, _: crate::elements::e45) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/flat_origin.rs");
