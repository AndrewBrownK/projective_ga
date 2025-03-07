use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// CircleRotorAligningOriginAtInfinity.
/// This variant of CircleRotorAligningOrigin exists at the Horizon.
#[repr(C)]
#[derive(Clone, Copy)]
pub union CircleRotorAligningOriginAtInfinity {
    groups: CircleRotorAligningOriginAtInfinityGroups,
    /// e415, e425, e435, 0, e235, e315, e125, e12345
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct CircleRotorAligningOriginAtInfinityGroups {
    /// e415, e425, e435
    g0: Simd32x3,
    /// e235, e315, e125, e12345
    g1: Simd32x4,
}
impl CircleRotorAligningOriginAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e235: f32, e315: f32, e125: f32, e12345: f32) -> Self {
        Self {
            elements: [e415, e425, e435, 0.0, e235, e315, e125, e12345],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4) -> Self {
        Self {
            groups: CircleRotorAligningOriginAtInfinityGroups { g0, g1 },
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
    pub fn group1(&self) -> Simd32x4 {
        unsafe { self.groups.g1 }
    }
    #[inline(always)]
    pub fn group1_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g1 }
    }
}
const CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP: [usize; 7] = [0, 1, 2, 4, 5, 6, 7];
impl std::ops::Index<usize> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<CircleRotorAligningOriginAtInfinity> for [f32; 7] {
    fn from(vector: CircleRotorAligningOriginAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 7]> for CircleRotorAligningOriginAtInfinity {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for CircleRotorAligningOriginAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("CircleRotorAligningOriginAtInfinity")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e235", &self[3])
            .field("e315", &self[4])
            .field("e125", &self[5])
            .field("e12345", &self[6])
            .finish()
    }
}

impl CircleRotorAligningOriginAtInfinity {
    pub const LEN: usize = 7;
}

impl nearly::NearlyEqEps<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {
    fn nearly_eq_eps(&self, other: &CircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {
    fn nearly_eq_ulps(&self, other: &CircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyEq<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyOrdUlps<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {
    fn nearly_lt_ulps(&self, other: &CircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &CircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {
    fn nearly_lt_eps(&self, other: &CircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &CircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyOrd<CircleRotorAligningOriginAtInfinity, f32, f32> for CircleRotorAligningOriginAtInfinity {}

impl CircleRotorAligningOriginAtInfinity {
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

impl PartialOrd for CircleRotorAligningOriginAtInfinity {
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
impl Ord for CircleRotorAligningOriginAtInfinity {
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
impl PartialEq for CircleRotorAligningOriginAtInfinity {
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
impl Eq for CircleRotorAligningOriginAtInfinity {}
impl std::hash::Hash for CircleRotorAligningOriginAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for CircleRotorAligningOriginAtInfinity {}
unsafe impl bytemuck::Pod for CircleRotorAligningOriginAtInfinity {}
impl encase::ShaderType for CircleRotorAligningOriginAtInfinity {
    type ExtraMetadata = <CircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <CircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <CircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <CircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <CircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for CircleRotorAligningOriginAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CircleRotorAligningOriginAtInfinity", 7)?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e12345", &self[crate::elements::e12345])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircleRotorAligningOriginAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum CircleRotorAligningOriginAtInfinityField {
            e415,
            e425,
            e435,
            e235,
            e315,
            e125,
            e12345,
        }
        struct CircleRotorAligningOriginAtInfinityVisitor;
        impl<'de> Visitor<'de> for CircleRotorAligningOriginAtInfinityVisitor {
            type Value = CircleRotorAligningOriginAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CircleRotorAligningOriginAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<CircleRotorAligningOriginAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e12345 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        CircleRotorAligningOriginAtInfinityField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        CircleRotorAligningOriginAtInfinityField::e12345 => {
                            if e12345.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12345"));
                            }
                            e12345 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = CircleRotorAligningOriginAtInfinity::from([0.0; 7]);
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e12345] = e12345.ok_or_else(|| serde::de::Error::missing_field("e12345"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e415", "e425", "e435", "e235", "e315", "e125", "e12345"];
        deserializer.deserialize_struct("CircleRotorAligningOriginAtInfinity", FIELDS, CircleRotorAligningOriginAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e415> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e235> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e315> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e125> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12345> for CircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12345) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12345> for CircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e12345) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/circle_rotor_aligning_origin_at_infinity.rs");
