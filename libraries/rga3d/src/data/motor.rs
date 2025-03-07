use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// Motor
#[repr(C)]
#[derive(Clone, Copy)]
pub union Motor {
    groups: MotorGroups,
    /// e41, e42, e43, e1234, e23, e31, e12, scalar
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct MotorGroups {
    /// e41, e42, e43, e1234
    g0: Simd32x4,
    /// e23, e31, e12, scalar
    g1: Simd32x4,
}
impl Motor {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e1234: f32, e23: f32, e31: f32, e12: f32, scalar: f32) -> Self {
        Self {
            elements: [e41, e42, e43, e1234, e23, e31, e12, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x4) -> Self {
        Self { groups: MotorGroups { g0, g1 } }
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
const MOTOR_INDEX_REMAP: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
impl std::ops::Index<usize> for Motor {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for Motor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[MOTOR_INDEX_REMAP[index]] }
    }
}
impl From<Motor> for [f32; 8] {
    fn from(vector: Motor) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 8]> for Motor {
    fn from(array: [f32; 8]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for Motor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("Motor")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e1234", &self[3])
            .field("e23", &self[4])
            .field("e31", &self[5])
            .field("e12", &self[6])
            .field("scalar", &self[7])
            .finish()
    }
}

impl Motor {
    pub const LEN: usize = 8;
}

impl nearly::NearlyEqEps<Motor, f32, f32> for Motor {
    fn nearly_eq_eps(&self, other: &Motor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<Motor, f32, f32> for Motor {
    fn nearly_eq_ulps(&self, other: &Motor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<Motor, f32, f32> for Motor {}
impl nearly::NearlyEq<Motor, f32, f32> for Motor {}
impl nearly::NearlyOrdUlps<Motor, f32, f32> for Motor {
    fn nearly_lt_ulps(&self, other: &Motor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &Motor, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<Motor, f32, f32> for Motor {
    fn nearly_lt_eps(&self, other: &Motor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &Motor, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<Motor, f32, f32> for Motor {}
impl nearly::NearlyOrd<Motor, f32, f32> for Motor {}

impl Motor {
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

impl PartialOrd for Motor {
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
impl Ord for Motor {
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
impl PartialEq for Motor {
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
impl Eq for Motor {}
impl std::hash::Hash for Motor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for Motor {}
unsafe impl bytemuck::Pod for Motor {}
impl encase::ShaderType for Motor {
    type ExtraMetadata = <MotorGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <MotorGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <MotorGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <MotorGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <MotorGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for Motor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Motor", 8)?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e1234", &self[crate::elements::e1234])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for Motor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum MotorField {
            e41,
            e42,
            e43,
            e1234,
            e23,
            e31,
            e12,
            scalar,
        }
        struct MotorVisitor;
        impl<'de> Visitor<'de> for MotorVisitor {
            type Value = Motor;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Motor")
            }
            fn visit_map<V>(self, mut map: V) -> Result<Motor, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e1234 = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut scalar = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        MotorField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        MotorField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        MotorField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        MotorField::e1234 => {
                            if e1234.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1234"));
                            }
                            e1234 = Some(map.next_value()?);
                        }

                        MotorField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        MotorField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        MotorField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        MotorField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = Motor::from([0.0; 8]);
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e1234] = e1234.ok_or_else(|| serde::de::Error::missing_field("e1234"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e41", "e42", "e43", "e1234", "e23", "e31", "e12", "scalar"];
        deserializer.deserialize_struct("Motor", FIELDS, MotorVisitor)
    }
}
impl std::ops::Index<crate::elements::e41> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e1234> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e1234) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e23> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e31> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e12> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::scalar> for Motor {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for Motor {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for Motor {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for Motor {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e1234> for Motor {
    fn index_mut(&mut self, _: crate::elements::e1234) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for Motor {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for Motor {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for Motor {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for Motor {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[7]
    }
}
include!("./impls/motor.rs");
