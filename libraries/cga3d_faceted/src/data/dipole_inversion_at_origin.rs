use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DipoleInversionAtOrigin.
/// This variant of DipoleInversion is centered on the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union DipoleInversionAtOrigin {
    groups: DipoleInversionAtOriginGroups,
    /// e41, e42, e43, e3215, e15, e25, e35, e1234
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct DipoleInversionAtOriginGroups {
    /// e41, e42, e43, e3215
    g0: Simd32x4,
    /// e15, e25, e35, e1234
    g1: Simd32x4,
}
impl DipoleInversionAtOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e3215: f32, e15: f32, e25: f32, e35: f32, e1234: f32) -> Self {
        Self {
            elements: [e41, e42, e43, e3215, e15, e25, e35, e1234],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: DipoleInversionAtOriginGroups { g0, g1 },
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
    #[inline(always)]
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const DIPOLE_INVERSION_AT_ORIGIN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DIPOLE_INVERSION_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DIPOLE_INVERSION_AT_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<DipoleInversionAtOrigin> for [f32; 8] {
    fn from(vector: DipoleInversionAtOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for DipoleInversionAtOrigin {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for DipoleInversionAtOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("DipoleInversionAtOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e3215", &self[3])
            .field("e15", &self[4])
            .field("e25", &self[5])
            .field("e35", &self[6])
            .field("e1234", &self[7])
            .finish()
    }
}

impl DipoleInversionAtOrigin {
    pub const LEN: usize = 8;
}

impl nearly::NearlyEqEps<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {
    fn nearly_eq_eps(&self, other: &DipoleInversionAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {
    fn nearly_eq_ulps(&self, other: &DipoleInversionAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {}
impl nearly::NearlyEq<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {}
impl nearly::NearlyOrdUlps<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {
    fn nearly_lt_ulps(&self, other: &DipoleInversionAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &DipoleInversionAtOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {
    fn nearly_lt_eps(&self, other: &DipoleInversionAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &DipoleInversionAtOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {}
impl nearly::NearlyOrd<DipoleInversionAtOrigin, f32, f32> for DipoleInversionAtOrigin {}

impl DipoleInversionAtOrigin {
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

impl PartialOrd for DipoleInversionAtOrigin {
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
impl Ord for DipoleInversionAtOrigin {
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
impl PartialEq for DipoleInversionAtOrigin {
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
impl Eq for DipoleInversionAtOrigin {}
impl std::hash::Hash for DipoleInversionAtOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for DipoleInversionAtOrigin {}
unsafe impl bytemuck::Pod for DipoleInversionAtOrigin {}
impl encase::ShaderType for DipoleInversionAtOrigin {
    type ExtraMetadata = <DipoleInversionAtOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <DipoleInversionAtOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <DipoleInversionAtOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <DipoleInversionAtOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <DipoleInversionAtOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for DipoleInversionAtOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DipoleInversionAtOrigin", 8)?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for DipoleInversionAtOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum DipoleInversionAtOriginField {
            e41,
            e42,
            e43,
            e3215,
            e15,
            e25,
            e35,
            e1234,
        }
        struct DipoleInversionAtOriginVisitor;
        impl<'de> Visitor<'de> for DipoleInversionAtOriginVisitor {
            type Value = DipoleInversionAtOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DipoleInversionAtOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<DipoleInversionAtOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e3215 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e1234 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        DipoleInversionAtOriginField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        DipoleInversionAtOriginField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = DipoleInversionAtOrigin::from([0.0; 8]);
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e41", "e42", "e43", "e3215", "e15", "e25", "e35", "e1234"];
        deserializer.deserialize_struct("DipoleInversionAtOrigin", FIELDS, DipoleInversionAtOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e41> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e15> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e25> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e35> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e1234> for DipoleInversionAtOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for DipoleInversionAtOrigin {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/dipole_inversion_at_origin.rs");
