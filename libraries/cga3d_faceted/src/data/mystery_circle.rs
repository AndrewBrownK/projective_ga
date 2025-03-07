use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MysteryCircle.
/// TODO this is currently a mystery I'm investigating
#[repr(C)]
#[derive(Clone, Copy)]
pub union MysteryCircle {
    groups: MysteryCircleGroups,
    /// e415, e425, e435, e321
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MysteryCircleGroups {
    /// e415, e425, e435, e321
    g0: Simd32x4,
}
impl MysteryCircle {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e321: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: MysteryCircleGroups { g0 },
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
const MYSTERY_CIRCLE_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for MysteryCircle {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_CIRCLE_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryCircle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_CIRCLE_INDEX_REMAP[index]] }
    }
}
impl From<MysteryCircle> for [f32; 4] {
    fn from(vector: MysteryCircle) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for MysteryCircle {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for MysteryCircle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryCircle")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e321", &self[3])
            .finish()
    }
}

impl MysteryCircle {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<MysteryCircle, f32, f32> for MysteryCircle {
    fn nearly_eq_eps(&self, other: &MysteryCircle, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<MysteryCircle, f32, f32> for MysteryCircle {
    fn nearly_eq_ulps(&self, other: &MysteryCircle, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<MysteryCircle, f32, f32> for MysteryCircle {}
impl nearly::NearlyEq<MysteryCircle, f32, f32> for MysteryCircle {}
impl nearly::NearlyOrdUlps<MysteryCircle, f32, f32> for MysteryCircle {
    fn nearly_lt_ulps(&self, other: &MysteryCircle, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &MysteryCircle, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<MysteryCircle, f32, f32> for MysteryCircle {
    fn nearly_lt_eps(&self, other: &MysteryCircle, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &MysteryCircle, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<MysteryCircle, f32, f32> for MysteryCircle {}
impl nearly::NearlyOrd<MysteryCircle, f32, f32> for MysteryCircle {}

impl MysteryCircle {
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

impl PartialOrd for MysteryCircle {
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
impl Ord for MysteryCircle {
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
impl PartialEq for MysteryCircle {
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
impl Eq for MysteryCircle {}
impl std::hash::Hash for MysteryCircle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MysteryCircle {}
unsafe impl bytemuck::Pod for MysteryCircle {}
impl encase::ShaderType for MysteryCircle {
    type ExtraMetadata = <MysteryCircleGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MysteryCircleGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MysteryCircleGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MysteryCircleGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MysteryCircleGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MysteryCircle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MysteryCircle", 4)?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MysteryCircle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MysteryCircleField {
            e415,
            e425,
            e435,
            e321,
        }
        struct MysteryCircleVisitor;
        impl<'de> Visitor<'de> for MysteryCircleVisitor {
            type Value = MysteryCircle;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MysteryCircle")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MysteryCircle, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MysteryCircleField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        MysteryCircleField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        MysteryCircleField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        MysteryCircleField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MysteryCircle::from([0.0; 4]);
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e415", "e425", "e435", "e321"];
        deserializer.deserialize_struct("MysteryCircle", FIELDS, MysteryCircleVisitor)
    }
}
impl std::ops::Index<crate::elements::e415> for MysteryCircle {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for MysteryCircle {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for MysteryCircle {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for MysteryCircle {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MysteryCircle {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MysteryCircle {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MysteryCircle {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MysteryCircle {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/mystery_circle.rs");
