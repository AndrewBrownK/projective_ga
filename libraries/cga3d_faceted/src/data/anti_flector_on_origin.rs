use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiFlectorOnOrigin.
/// This variant of VersorEvenOrthogonalOrigin is the Dual to FlectorOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorEvenOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// FlectorOnOrigin with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiFlectorOnOrigin {
    groups: AntiFlectorOnOriginGroups,
    /// e321, e1, e2, e3
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiFlectorOnOriginGroups {
    /// e321, e1, e2, e3
    g0: Simd32x4,
}
impl AntiFlectorOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e321: f32, e1: f32, e2: f32, e3: f32) -> Self {
        Self { elements: [e321, e1, e2, e3] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: AntiFlectorOnOriginGroups { g0 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x4 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g0 }
    }
}
const ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_FLECTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiFlectorOnOrigin> for [f32; 4] {
    fn from(vector: AntiFlectorOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for AntiFlectorOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for AntiFlectorOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiFlectorOnOrigin")
            .field("e321", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .finish()
    }
}

impl AntiFlectorOnOrigin {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {
    fn nearly_eq_eps(&self, other: &AntiFlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {
    fn nearly_eq_ulps(&self, other: &AntiFlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {}
impl nearly::NearlyEq<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {}
impl nearly::NearlyOrdUlps<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {
    fn nearly_lt_ulps(&self, other: &AntiFlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiFlectorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {
    fn nearly_lt_eps(&self, other: &AntiFlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiFlectorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {}
impl nearly::NearlyOrd<AntiFlectorOnOrigin, f32, f32> for AntiFlectorOnOrigin {}

impl AntiFlectorOnOrigin {
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

impl PartialOrd for AntiFlectorOnOrigin {
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
impl Ord for AntiFlectorOnOrigin {
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
impl PartialEq for AntiFlectorOnOrigin {
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
impl Eq for AntiFlectorOnOrigin {}
impl std::hash::Hash for AntiFlectorOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiFlectorOnOrigin {}
unsafe impl bytemuck::Pod for AntiFlectorOnOrigin {}
impl encase::ShaderType for AntiFlectorOnOrigin {
    type ExtraMetadata = <AntiFlectorOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiFlectorOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiFlectorOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiFlectorOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiFlectorOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiFlectorOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiFlectorOnOrigin", 4)?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiFlectorOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiFlectorOnOriginField {
            e321,
            e1,
            e2,
            e3,
        }
        struct AntiFlectorOnOriginVisitor;
        impl<'de> Visitor<'de> for AntiFlectorOnOriginVisitor {
            type Value = AntiFlectorOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiFlectorOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiFlectorOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e321 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiFlectorOnOriginField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        AntiFlectorOnOriginField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        AntiFlectorOnOriginField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        AntiFlectorOnOriginField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiFlectorOnOrigin::from([0.0; 4]);
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e321", "e1", "e2", "e3"];
        deserializer.deserialize_struct("AntiFlectorOnOrigin", FIELDS, AntiFlectorOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e321> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiFlectorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiFlectorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_flector_on_origin.rs");
