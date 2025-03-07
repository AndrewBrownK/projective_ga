use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiScalar
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiScalar {
    groups: AntiScalarGroups,
    /// e12345, 0, 0, 0
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiScalarGroups {
    /// e12345
    g0: f32,
}
impl AntiScalar {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e12345: f32) -> Self {
        Self {
            elements: [e12345, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: f32) -> Self {
        Self { groups: AntiScalarGroups { g0 } }
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
const ANTI_SCALAR_INDEX_REMAP: [usize; 1] = [0];
impl std::ops::Index<usize> for AntiScalar {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_SCALAR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiScalar {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_SCALAR_INDEX_REMAP[index]] }
    }
}
impl From<AntiScalar> for [f32; 1] {
    fn from(vector: AntiScalar) -> Self {
        unsafe { [vector.elements[0]] }
    }
}
impl From<[f32; 1]> for AntiScalar {
    fn from(array: [f32; 1]) -> Self {
        Self {
            elements: [array[0], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for AntiScalar {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.debug_struct("AntiScalar").field("e12345", &self[0]).finish()
    }
}

impl AntiScalar {
    pub const LEN: usize = 1;
}

impl nearly::NearlyEqEps<AntiScalar, f32, f32> for AntiScalar {
    fn nearly_eq_eps(&self, other: &AntiScalar, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiScalar, f32, f32> for AntiScalar {
    fn nearly_eq_ulps(&self, other: &AntiScalar, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiScalar, f32, f32> for AntiScalar {}
impl nearly::NearlyEq<AntiScalar, f32, f32> for AntiScalar {}
impl nearly::NearlyOrdUlps<AntiScalar, f32, f32> for AntiScalar {
    fn nearly_lt_ulps(&self, other: &AntiScalar, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiScalar, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiScalar, f32, f32> for AntiScalar {
    fn nearly_lt_eps(&self, other: &AntiScalar, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiScalar, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiScalar, f32, f32> for AntiScalar {}
impl nearly::NearlyOrd<AntiScalar, f32, f32> for AntiScalar {}

impl AntiScalar {
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

impl PartialOrd for AntiScalar {
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
impl Ord for AntiScalar {
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
impl PartialEq for AntiScalar {
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
impl Eq for AntiScalar {}
impl std::hash::Hash for AntiScalar {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiScalar {}
unsafe impl bytemuck::Pod for AntiScalar {}
impl encase::ShaderType for AntiScalar {
    type ExtraMetadata = <AntiScalarGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiScalarGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiScalarGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiScalarGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiScalarGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiScalar {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiScalar", 1)?;
        state.serialize_field("e12345", &self[crate::elements::e12345])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiScalar {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiScalarField {
            e12345,
        }
        struct AntiScalarVisitor;
        impl<'de> Visitor<'de> for AntiScalarVisitor {
            type Value = AntiScalar;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiScalar")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiScalar, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e12345 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiScalarField::e12345 => {
                            if e12345.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12345"));
                            }
                            e12345 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiScalar::from([0.0; 1]);
                result[crate::elements::e12345] = e12345.ok_or_else(|| serde::de::Error::missing_field("e12345"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e12345"];
        deserializer.deserialize_struct("AntiScalar", FIELDS, AntiScalarVisitor)
    }
}
impl std::ops::Index<crate::elements::e12345> for AntiScalar {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for AntiScalar {
    fn index_mut(&mut self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[0]
    }
}
include!("./impls/anti_scalar.rs");
