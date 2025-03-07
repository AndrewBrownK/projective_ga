use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiMotorOnOrigin.
/// This variant of MysteryVersorOdd is the Dual to MotorOnOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a MysteryVersorOdd,
/// but an imaginary radius, and a spacial presence in the shape of a
/// MotorOnOrigin with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiMotorOnOrigin {
    groups: AntiMotorOnOriginGroups,
    /// e23, e31, e12, scalar
    elements: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiMotorOnOriginGroups {
    /// e23, e31, e12, scalar
    g0: Simd32x4,
}
impl AntiMotorOnOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, scalar: f32) -> Self {
        Self {
            elements: [e23, e31, e12, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x4) -> Self {
        Self {
            groups: AntiMotorOnOriginGroups { g0 },
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
const ANTI_MOTOR_ON_ORIGIN_INDEX_REMAP: [usize; 4] = [0, 1, 2, 3];
impl std::ops::Index<usize> for AntiMotorOnOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_MOTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiMotorOnOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_MOTOR_ON_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiMotorOnOrigin> for [f32; 4] {
    fn from(vector: AntiMotorOnOrigin) -> Self {
        unsafe { [vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3]] }
    }
}
impl From<[f32; 4]> for AntiMotorOnOrigin {
    fn from(array: [f32; 4]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3]],
        }
    }
}
impl std::fmt::Debug for AntiMotorOnOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiMotorOnOrigin")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("scalar", &self[3])
            .finish()
    }
}

impl AntiMotorOnOrigin {
    pub const LEN: usize = 4;
}

impl nearly::NearlyEqEps<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {
    fn nearly_eq_eps(&self, other: &AntiMotorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {
    fn nearly_eq_ulps(&self, other: &AntiMotorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {}
impl nearly::NearlyEq<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {}
impl nearly::NearlyOrdUlps<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {
    fn nearly_lt_ulps(&self, other: &AntiMotorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiMotorOnOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {
    fn nearly_lt_eps(&self, other: &AntiMotorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiMotorOnOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {}
impl nearly::NearlyOrd<AntiMotorOnOrigin, f32, f32> for AntiMotorOnOrigin {}

impl AntiMotorOnOrigin {
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

impl PartialOrd for AntiMotorOnOrigin {
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
impl Ord for AntiMotorOnOrigin {
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
impl PartialEq for AntiMotorOnOrigin {
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
impl Eq for AntiMotorOnOrigin {}
impl std::hash::Hash for AntiMotorOnOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiMotorOnOrigin {}
unsafe impl bytemuck::Pod for AntiMotorOnOrigin {}
impl encase::ShaderType for AntiMotorOnOrigin {
    type ExtraMetadata = <AntiMotorOnOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiMotorOnOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiMotorOnOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiMotorOnOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiMotorOnOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiMotorOnOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiMotorOnOrigin", 4)?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiMotorOnOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiMotorOnOriginField {
            e23,
            e31,
            e12,
            scalar,
        }
        struct AntiMotorOnOriginVisitor;
        impl<'de> Visitor<'de> for AntiMotorOnOriginVisitor {
            type Value = AntiMotorOnOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiMotorOnOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiMotorOnOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut scalar = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiMotorOnOriginField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        AntiMotorOnOriginField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        AntiMotorOnOriginField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        AntiMotorOnOriginField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiMotorOnOrigin::from([0.0; 4]);
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e23", "e31", "e12", "scalar"];
        deserializer.deserialize_struct("AntiMotorOnOrigin", FIELDS, AntiMotorOnOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e23> for AntiMotorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiMotorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiMotorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiMotorOnOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiMotorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiMotorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiMotorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiMotorOnOrigin {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[3]
    }
}
include!("./impls/anti_motor_on_origin.rs");
