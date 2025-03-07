use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MysteryCircleRotor.
/// TODO this is currently a mystery I'm investigating
#[repr(C)]
#[derive(Clone, Copy)]
pub union MysteryCircleRotor {
    groups: MysteryCircleRotorGroups,
    /// e415, e425, e435, e321, e12345, 0, 0, 0
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MysteryCircleRotorGroups {
    /// e415, e425, e435, e321
    g0: Simd32x4,
    /// e12345
    g1: f32,
}
impl MysteryCircleRotor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e321: f32, e12345: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e321, e12345, 0.0, 0.0, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: f32) -> Self {
        Self {
            groups: MysteryCircleRotorGroups { g0, g1 },
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
    pub fn group1(&self) -> f32 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut f32 {
        unsafe { &mut self.groups.g1 }
    }
}
const MYSTERY_CIRCLE_ROTOR_INDEX_REMAP: [usize; 5] = [0, 1, 2, 3, 4];
impl std::ops::Index<usize> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryCircleRotor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_CIRCLE_ROTOR_INDEX_REMAP[index]] }
    }
}
impl From<MysteryCircleRotor> for [f32; 5] {
    fn from(vector: MysteryCircleRotor) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4]] }
    }
}
impl From<[f32; 5]> for MysteryCircleRotor {
    fn from(array: [f32; 5]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], 0.0, 0.0, 0.0],
        }
    }
}
impl std::fmt::Debug for MysteryCircleRotor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryCircleRotor")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e321", &self[3])
            .field("e12345", &self[4])
            .finish()
    }
}

impl MysteryCircleRotor {
    pub const LEN: usize = 5;
}

impl nearly::NearlyEqEps<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {
    fn nearly_eq_eps(&self, other: &MysteryCircleRotor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {
    fn nearly_eq_ulps(&self, other: &MysteryCircleRotor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {}
impl nearly::NearlyEq<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {}
impl nearly::NearlyOrdUlps<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {
    fn nearly_lt_ulps(&self, other: &MysteryCircleRotor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &MysteryCircleRotor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {
    fn nearly_lt_eps(&self, other: &MysteryCircleRotor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &MysteryCircleRotor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {}
impl nearly::NearlyOrd<MysteryCircleRotor, f32, f32> for MysteryCircleRotor {}

impl MysteryCircleRotor {
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

impl PartialOrd for MysteryCircleRotor {
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
impl Ord for MysteryCircleRotor {
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
impl PartialEq for MysteryCircleRotor {
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
impl Eq for MysteryCircleRotor {}
impl std::hash::Hash for MysteryCircleRotor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MysteryCircleRotor {}
unsafe impl bytemuck::Pod for MysteryCircleRotor {}
impl encase::ShaderType for MysteryCircleRotor {
    type ExtraMetadata = <MysteryCircleRotorGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MysteryCircleRotorGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MysteryCircleRotorGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MysteryCircleRotorGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MysteryCircleRotorGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MysteryCircleRotor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MysteryCircleRotor", 5)?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e12345", &self[crate::elements::e12345])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MysteryCircleRotor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MysteryCircleRotorField {
            e415,
            e425,
            e435,
            e321,
            e12345,
        }
        struct MysteryCircleRotorVisitor;
        impl<'de> Visitor<'de> for MysteryCircleRotorVisitor {
            type Value = MysteryCircleRotor;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MysteryCircleRotor")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MysteryCircleRotor, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;
                let mut e12345 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MysteryCircleRotorField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        MysteryCircleRotorField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        MysteryCircleRotorField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        MysteryCircleRotorField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        MysteryCircleRotorField::e12345 => {
                            if e12345.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12345"));
                            }
                            e12345 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MysteryCircleRotor::from([0.0; 5]);
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e12345] = e12345.ok_or_else(|| serde::de::Error::missing_field("e12345"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e415", "e425", "e435", "e321", "e12345"];
        deserializer.deserialize_struct("MysteryCircleRotor", FIELDS, MysteryCircleRotorVisitor)
    }
}
impl std::ops::Index<crate::elements::e415> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e12345> for MysteryCircleRotor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MysteryCircleRotor {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MysteryCircleRotor {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MysteryCircleRotor {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MysteryCircleRotor {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MysteryCircleRotor {
    fn index_mut(&mut self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[4]
    }
}
include!("./impls/mystery_circle_rotor.rs");
