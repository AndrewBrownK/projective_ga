use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// MotorAtInfinity.
/// This variant of Motor exists in the Horizon.
#[repr(C)]
#[derive(Clone, Copy)]
pub union MotorAtInfinity {
    groups: MotorAtInfinityGroups,
    /// e235, e315, e125, e5
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MotorAtInfinityGroups {
    /// e235, e315, e125, e5
    g0: Simd32x4,
}
impl MotorAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e235: f32, e315: f32, e125: f32, e5: f32) -> Self {
        Self { elements: [e235, e315, e125, e5] }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: MotorAtInfinityGroups { g0 },
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
const MOTOR_AT_INFINITY_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for MotorAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for MotorAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTOR_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<MotorAtInfinity> for [f32; 4] {
    fn from(vector: MotorAtInfinity) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for MotorAtInfinity {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for MotorAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("MotorAtInfinity")
            .field("e235", &self[0])
            .field("e315", &self[1])
            .field("e125", &self[2])
            .field("e5", &self[3])
            .finish()
    }
}

impl MotorAtInfinity {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<MotorAtInfinity, f32, f32> for MotorAtInfinity {
    fn nearly_eq_eps(&self, other: &MotorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<MotorAtInfinity, f32, f32> for MotorAtInfinity {
    fn nearly_eq_ulps(&self, other: &MotorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<MotorAtInfinity, f32, f32> for MotorAtInfinity {}
impl nearly::NearlyEq<MotorAtInfinity, f32, f32> for MotorAtInfinity {}
impl nearly::NearlyOrdUlps<MotorAtInfinity, f32, f32> for MotorAtInfinity {
    fn nearly_lt_ulps(&self, other: &MotorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &MotorAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<MotorAtInfinity, f32, f32> for MotorAtInfinity {
    fn nearly_lt_eps(&self, other: &MotorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &MotorAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<MotorAtInfinity, f32, f32> for MotorAtInfinity {}
impl nearly::NearlyOrd<MotorAtInfinity, f32, f32> for MotorAtInfinity {}

impl MotorAtInfinity {
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

impl PartialOrd for MotorAtInfinity {
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
impl Ord for MotorAtInfinity {
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
impl PartialEq for MotorAtInfinity {
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
impl Eq for MotorAtInfinity {}
impl std::hash::Hash for MotorAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for MotorAtInfinity {}
unsafe impl bytemuck::Pod for MotorAtInfinity {}
impl encase::ShaderType for MotorAtInfinity {
    type ExtraMetadata = <MotorAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MotorAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MotorAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MotorAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MotorAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for MotorAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MotorAtInfinity", 4)?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for MotorAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MotorAtInfinityField {
            e235,
            e315,
            e125,
            e5,
        }
        struct MotorAtInfinityVisitor;
        impl<'de> Visitor<'de> for MotorAtInfinityVisitor {
            type Value = MotorAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MotorAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<MotorAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MotorAtInfinityField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        MotorAtInfinityField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        MotorAtInfinityField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        MotorAtInfinityField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = MotorAtInfinity::from([0.0; 4]);
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e235", "e315", "e125", "e5"];
        deserializer.deserialize_struct("MotorAtInfinity", FIELDS, MotorAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e235> for MotorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e315> for MotorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e125> for MotorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e5> for MotorAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for MotorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for MotorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for MotorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for MotorAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/motor_at_infinity.rs");
