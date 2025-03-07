use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MysteryVersorEven.
/// TODO this is currently a mystery I'm investigating
#[repr(C)]
#[derive(Clone, Copy)]
pub union MysteryVersorEven {
    groups: MysteryVersorEvenGroups,
    /// e12345, e1, e2, e3, e415, e425, e435, e321
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MysteryVersorEvenGroups {
    /// e12345, e1, e2, e3
    g0: Simd32x4,
    /// e415, e425, e435, e321
    g1: Simd32x4,
}
impl MysteryVersorEven {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e12345: f32, e1: f32, e2: f32, e3: f32, e415: f32, e425: f32, e435: f32, e321: f32) -> Self {
        Self {
            elements: [e12345, e1, e2, e3, e415, e425, e435, e321],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self {
            groups: MysteryVersorEvenGroups { g0, g1 },
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
const MYSTERY_VERSOR_EVEN_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MYSTERY_VERSOR_EVEN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MysteryVersorEven {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MYSTERY_VERSOR_EVEN_INDEX_REMAP[index]] }
    }
}
impl From<MysteryVersorEven> for [f32; 8] {
    fn from(vector: MysteryVersorEven) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for MysteryVersorEven {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for MysteryVersorEven {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MysteryVersorEven")
            .field("e12345", &self[0])
            .field("e1", &self[1])
            .field("e2", &self[2])
            .field("e3", &self[3])
            .field("e415", &self[4])
            .field("e425", &self[5])
            .field("e435", &self[6])
            .field("e321", &self[7])
            .finish()
    }
}

impl MysteryVersorEven {
    pub const LEN: usize = 8;
}

impl nearly::NearlyEqEps<MysteryVersorEven, f32, f32> for MysteryVersorEven {
    fn nearly_eq_eps(&self, other: &MysteryVersorEven, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<MysteryVersorEven, f32, f32> for MysteryVersorEven {
    fn nearly_eq_ulps(&self, other: &MysteryVersorEven, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<MysteryVersorEven, f32, f32> for MysteryVersorEven {}
impl nearly::NearlyEq<MysteryVersorEven, f32, f32> for MysteryVersorEven {}
impl nearly::NearlyOrdUlps<MysteryVersorEven, f32, f32> for MysteryVersorEven {
    fn nearly_lt_ulps(&self, other: &MysteryVersorEven, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &MysteryVersorEven, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<MysteryVersorEven, f32, f32> for MysteryVersorEven {
    fn nearly_lt_eps(&self, other: &MysteryVersorEven, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &MysteryVersorEven, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<MysteryVersorEven, f32, f32> for MysteryVersorEven {}
impl nearly::NearlyOrd<MysteryVersorEven, f32, f32> for MysteryVersorEven {}

impl MysteryVersorEven {
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

impl PartialOrd for MysteryVersorEven {
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
impl Ord for MysteryVersorEven {
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
impl PartialEq for MysteryVersorEven {
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
impl Eq for MysteryVersorEven {}
impl std::hash::Hash for MysteryVersorEven {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MysteryVersorEven {}
unsafe impl bytemuck::Pod for MysteryVersorEven {}
impl encase::ShaderType for MysteryVersorEven {
    type ExtraMetadata = <MysteryVersorEvenGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MysteryVersorEvenGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MysteryVersorEvenGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MysteryVersorEvenGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MysteryVersorEvenGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MysteryVersorEven {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MysteryVersorEven", 8)?;
        state.serialize_field("e12345", &self[crate::elements::e12345])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MysteryVersorEven {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MysteryVersorEvenField {
            e12345,
            e1,
            e2,
            e3,
            e415,
            e425,
            e435,
            e321,
        }
        struct MysteryVersorEvenVisitor;
        impl<'de> Visitor<'de> for MysteryVersorEvenVisitor {
            type Value = MysteryVersorEven;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MysteryVersorEven")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MysteryVersorEven, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e12345 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MysteryVersorEvenField::e12345 => {
                            if e12345.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12345"));
                            }
                            e12345 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        MysteryVersorEvenField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MysteryVersorEven::from([0.0; 8]);
                result[crate::elements::e12345] = e12345.ok_or_else(|| serde::de::Error::missing_field("e12345"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e12345", "e1", "e2", "e3", "e415", "e425", "e435", "e321"];
        deserializer.deserialize_struct("MysteryVersorEven", FIELDS, MysteryVersorEvenVisitor)
    }
}
impl std::ops::Index<crate::elements::e12345> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e1> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e2> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e3> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e415> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e425> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e435> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e321> for MysteryVersorEven {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for MysteryVersorEven {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/mystery_versor_even.rs");
