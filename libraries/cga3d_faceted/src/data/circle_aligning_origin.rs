use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CircleAligningOrigin.
/// This variant of Circle has a Carrier that intersects the Origin.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CircleAligningOrigin {
    groups: CircleAligningOriginGroups,
    /// e423, e431, e412, 0, e415, e425, e435, 0, e235, e315, e125, 0
    elements: [f32; 12],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct CircleAligningOriginGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e415, e425, e435
    g1: Simd32x3,
    /// e235, e315, e125
    g2: Simd32x3,
}
impl CircleAligningOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e423: f32, e431: f32, e412: f32, e415: f32, e425: f32, e435: f32, e235: f32, e315: f32, e125: f32) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e415, e425, e435, 0.0, e235, e315, e125, 0.0],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x3) -> Self {
        Self {
            groups: CircleAligningOriginGroups { g0, g1, g2 },
        }
    }
    #[inline(always)]
    pub fn group0(&self) -> Simd32x3 {
        unsafe { self.groups.g0 }
    }
    #[inline(always)]
    pub fn group0_mut(&mut self) -> &mut Simd32x3 {
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
    pub fn group2(&self) -> Simd32x3 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x3 {
        unsafe { &mut self.groups.g2 }
    }
}
const CIRCLE_ALIGNING_ORIGIN_INDEX_REMAP: [usize; 9] = [0, 1, 2, 4, 5, 6, 8, 9, 10];
impl std::ops::Index<usize> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleAligningOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<CircleAligningOrigin> for [f32; 9] {
    fn from(vector: CircleAligningOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9],
                vector.elements[10],
            ]
        }
    }
}
impl From<[f32; 9]> for CircleAligningOrigin {
    fn from(array: [f32; 9]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0, array[2], array[3], array[4], 0.0],
        }
    }
}
impl std::fmt::Debug for CircleAligningOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleAligningOrigin")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e415", &self[3])
            .field("e425", &self[4])
            .field("e435", &self[5])
            .field("e235", &self[6])
            .field("e315", &self[7])
            .field("e125", &self[8])
            .finish()
    }
}

impl CircleAligningOrigin {
    pub const LEN: usize = 9;
}

impl nearly::NearlyEqEps<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {
    fn nearly_eq_eps(&self, other: &CircleAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {
    fn nearly_eq_ulps(&self, other: &CircleAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {}
impl nearly::NearlyEq<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {}
impl nearly::NearlyOrdUlps<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {
    fn nearly_lt_ulps(&self, other: &CircleAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &CircleAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {
    fn nearly_lt_eps(&self, other: &CircleAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &CircleAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {}
impl nearly::NearlyOrd<CircleAligningOrigin, f32, f32> for CircleAligningOrigin {}

impl CircleAligningOrigin {
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

impl PartialOrd for CircleAligningOrigin {
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
impl Ord for CircleAligningOrigin {
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
impl PartialEq for CircleAligningOrigin {
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
impl Eq for CircleAligningOrigin {}
impl std::hash::Hash for CircleAligningOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for CircleAligningOrigin {}
unsafe impl bytemuck::Pod for CircleAligningOrigin {}
impl encase::ShaderType for CircleAligningOrigin {
    type ExtraMetadata = <CircleAligningOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <CircleAligningOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <CircleAligningOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <CircleAligningOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <CircleAligningOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for CircleAligningOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CircleAligningOrigin", 9)?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircleAligningOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum CircleAligningOriginField {
            e423,
            e431,
            e412,
            e415,
            e425,
            e435,
            e235,
            e315,
            e125,
        }
        struct CircleAligningOriginVisitor;
        impl<'de> Visitor<'de> for CircleAligningOriginVisitor {
            type Value = CircleAligningOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CircleAligningOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<CircleAligningOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        CircleAligningOriginField::e423 => {
                            if e423.is_some() {
                                return Err(serde::de::Error::duplicate_field("e423"));
                            }
                            e423 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e431 => {
                            if e431.is_some() {
                                return Err(serde::de::Error::duplicate_field("e431"));
                            }
                            e431 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e412 => {
                            if e412.is_some() {
                                return Err(serde::de::Error::duplicate_field("e412"));
                            }
                            e412 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        CircleAligningOriginField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = CircleAligningOrigin::from([0.0; 9]);
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e423", "e431", "e412", "e415", "e425", "e435", "e235", "e315", "e125"];
        deserializer.deserialize_struct("CircleAligningOrigin", FIELDS, CircleAligningOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e423> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e415> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[8]
    }
}
include!("./impls/circle_aligning_origin.rs");
