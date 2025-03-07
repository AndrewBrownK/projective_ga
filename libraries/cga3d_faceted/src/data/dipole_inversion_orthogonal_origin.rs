use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// DipoleInversionOrthogonalOrigin.
/// This variant of DipoleInversion has a CoCarrier that intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union DipoleInversionOrthogonalOrigin {
    groups: DipoleInversionOrthogonalOriginGroups,
    /// e41, e42, e43, e3215, e23, e31, e12, 0, e15, e25, e35, e1234
    elements: [f32; 12],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct DipoleInversionOrthogonalOriginGroups {
    /// e41, e42, e43, e3215
    g0: Simd32x4,
    /// e23, e31, e12
    g1: Simd32x3,
    /// e15, e25, e35, e1234
    g2: Simd32x4,
}
impl DipoleInversionOrthogonalOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e3215: f32, e23: f32, e31: f32, e12: f32, e15: f32, e25: f32, e35: f32, e1234: f32) -> Self {
        Self {
            elements: [e41, e42, e43, e3215, e23, e31, e12, 0.0, e15, e25, e35, e1234],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self {
            groups: DipoleInversionOrthogonalOriginGroups { g0, g1, g2 },
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
    pub fn group1(&self) -> Simd32x3 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g1 }
    }
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const DIPOLE_INVERSION_ORTHOGONAL_ORIGIN_INDEX_REMAP: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
impl std::ops::Index<usize> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[DIPOLE_INVERSION_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[DIPOLE_INVERSION_ORTHOGONAL_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<DipoleInversionOrthogonalOrigin> for [f32; 11] {
    fn from(vector: DipoleInversionOrthogonalOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8],
                vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 11]> for DipoleInversionOrthogonalOrigin {
    fn from(array: [f32; 11]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0, array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for DipoleInversionOrthogonalOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("DipoleInversionOrthogonalOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e3215", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("e15", &self[7])
            .field("e25", &self[8])
            .field("e35", &self[9])
            .field("e1234", &self[10])
            .finish()
    }
}

impl DipoleInversionOrthogonalOrigin {
    pub const LEN: usize = 11;
}

impl nearly::NearlyEqEps<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {
    fn nearly_eq_eps(&self, other: &DipoleInversionOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {
    fn nearly_eq_ulps(&self, other: &DipoleInversionOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {}
impl nearly::NearlyEq<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {}
impl nearly::NearlyOrdUlps<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {
    fn nearly_lt_ulps(&self, other: &DipoleInversionOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &DipoleInversionOrthogonalOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {
    fn nearly_lt_eps(&self, other: &DipoleInversionOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &DipoleInversionOrthogonalOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {}
impl nearly::NearlyOrd<DipoleInversionOrthogonalOrigin, f32, f32> for DipoleInversionOrthogonalOrigin {}

impl DipoleInversionOrthogonalOrigin {
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

impl PartialOrd for DipoleInversionOrthogonalOrigin {
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
impl Ord for DipoleInversionOrthogonalOrigin {
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
impl PartialEq for DipoleInversionOrthogonalOrigin {
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
impl Eq for DipoleInversionOrthogonalOrigin {}
impl std::hash::Hash for DipoleInversionOrthogonalOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for DipoleInversionOrthogonalOrigin {}
unsafe impl bytemuck::Pod for DipoleInversionOrthogonalOrigin {}
impl encase::ShaderType for DipoleInversionOrthogonalOrigin {
    type ExtraMetadata = <DipoleInversionOrthogonalOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <DipoleInversionOrthogonalOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <DipoleInversionOrthogonalOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <DipoleInversionOrthogonalOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <DipoleInversionOrthogonalOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for DipoleInversionOrthogonalOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DipoleInversionOrthogonalOrigin", 11)?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e3215", &self[crate::elements::e3215])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for DipoleInversionOrthogonalOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum DipoleInversionOrthogonalOriginField {
            e41,
            e42,
            e43,
            e3215,
            e23,
            e31,
            e12,
            e15,
            e25,
            e35,
            e1234,
        }
        struct DipoleInversionOrthogonalOriginVisitor;
        impl<'de> Visitor<'de> for DipoleInversionOrthogonalOriginVisitor {
            type Value = DipoleInversionOrthogonalOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DipoleInversionOrthogonalOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<DipoleInversionOrthogonalOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e3215 = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut e1234 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        DipoleInversionOrthogonalOriginField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e3215 => {
                            if e3215.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3215"));
                            }
                            e3215 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        DipoleInversionOrthogonalOriginField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = DipoleInversionOrthogonalOrigin::from([0.0; 11]);
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e3215] = e3215.ok_or_else(|| serde::de::Error::missing_field("e3215"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e41", "e42", "e43", "e3215", "e23", "e31", "e12", "e15", "e25", "e35", "e1234"];
        deserializer.deserialize_struct("DipoleInversionOrthogonalOrigin", FIELDS, DipoleInversionOrthogonalOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e41> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3215> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e3215) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e15> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e25> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e35> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e1234> for DipoleInversionOrthogonalOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3215> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e3215) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for DipoleInversionOrthogonalOrigin {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[10]
    }
}
include!("./impls/dipole_inversion_orthogonal_origin.rs");
