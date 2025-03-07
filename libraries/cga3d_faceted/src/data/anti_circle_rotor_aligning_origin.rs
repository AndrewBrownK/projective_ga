use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiCircleRotorAligningOrigin.
/// This variant of VersorOddOrthogonalOrigin is the Dual to CircleRotorAligningOrigin. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOddOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// CircleRotorAligningOrigin with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiCircleRotorAligningOrigin {
    groups: AntiCircleRotorAligningOriginGroups,
    /// e41, e42, e43, 0, e23, e31, e12, 0, e15, e25, e35, scalar
    elements: [f32; 12],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiCircleRotorAligningOriginGroups {
    /// e41, e42, e43
    g0: Simd32x3,
    /// e23, e31, e12
    g1: Simd32x3,
    /// e15, e25, e35, scalar
    g2: Simd32x4,
}
impl AntiCircleRotorAligningOrigin {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e41: f32, e42: f32, e43: f32, e23: f32, e31: f32, e12: f32, e15: f32, e25: f32, e35: f32, scalar: f32) -> Self {
        Self {
            elements: [e41, e42, e43, 0.0, e23, e31, e12, 0.0, e15, e25, e35, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self {
            groups: AntiCircleRotorAligningOriginGroups { g0, g1, g2 },
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
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
}
const ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP: [usize; 10] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 11];
impl std::ops::Index<usize> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_INDEX_REMAP[index]] }
    }
}
impl From<AntiCircleRotorAligningOrigin> for [f32; 10] {
    fn from(vector: AntiCircleRotorAligningOrigin) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8], vector.elements[9],
                vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 10]> for AntiCircleRotorAligningOrigin {
    fn from(array: [f32; 10]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], 0.0, array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for AntiCircleRotorAligningOrigin {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiCircleRotorAligningOrigin")
            .field("e41", &self[0])
            .field("e42", &self[1])
            .field("e43", &self[2])
            .field("e23", &self[3])
            .field("e31", &self[4])
            .field("e12", &self[5])
            .field("e15", &self[6])
            .field("e25", &self[7])
            .field("e35", &self[8])
            .field("scalar", &self[9])
            .finish()
    }
}

impl AntiCircleRotorAligningOrigin {
    pub const LEN: usize = 10;
}

impl nearly::NearlyEqEps<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {
    fn nearly_eq_eps(&self, other: &AntiCircleRotorAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {
    fn nearly_eq_ulps(&self, other: &AntiCircleRotorAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {}
impl nearly::NearlyEq<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {}
impl nearly::NearlyOrdUlps<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {
    fn nearly_lt_ulps(&self, other: &AntiCircleRotorAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiCircleRotorAligningOrigin, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {
    fn nearly_lt_eps(&self, other: &AntiCircleRotorAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiCircleRotorAligningOrigin, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {}
impl nearly::NearlyOrd<AntiCircleRotorAligningOrigin, f32, f32> for AntiCircleRotorAligningOrigin {}

impl AntiCircleRotorAligningOrigin {
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

impl PartialOrd for AntiCircleRotorAligningOrigin {
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
impl Ord for AntiCircleRotorAligningOrigin {
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
impl PartialEq for AntiCircleRotorAligningOrigin {
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
impl Eq for AntiCircleRotorAligningOrigin {}
impl std::hash::Hash for AntiCircleRotorAligningOrigin {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiCircleRotorAligningOrigin {}
unsafe impl bytemuck::Pod for AntiCircleRotorAligningOrigin {}
impl encase::ShaderType for AntiCircleRotorAligningOrigin {
    type ExtraMetadata = <AntiCircleRotorAligningOriginGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiCircleRotorAligningOriginGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiCircleRotorAligningOriginGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiCircleRotorAligningOriginGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiCircleRotorAligningOriginGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiCircleRotorAligningOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiCircleRotorAligningOrigin", 10)?;
        state.serialize_field("e41", &self[crate::elements::e41])?;
        state.serialize_field("e42", &self[crate::elements::e42])?;
        state.serialize_field("e43", &self[crate::elements::e43])?;
        state.serialize_field("e23", &self[crate::elements::e23])?;
        state.serialize_field("e31", &self[crate::elements::e31])?;
        state.serialize_field("e12", &self[crate::elements::e12])?;
        state.serialize_field("e15", &self[crate::elements::e15])?;
        state.serialize_field("e25", &self[crate::elements::e25])?;
        state.serialize_field("e35", &self[crate::elements::e35])?;
        state.serialize_field("scalar", &self[crate::elements::scalar])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiCircleRotorAligningOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiCircleRotorAligningOriginField {
            e41,
            e42,
            e43,
            e23,
            e31,
            e12,
            e15,
            e25,
            e35,
            scalar,
        }
        struct AntiCircleRotorAligningOriginVisitor;
        impl<'de> Visitor<'de> for AntiCircleRotorAligningOriginVisitor {
            type Value = AntiCircleRotorAligningOrigin;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiCircleRotorAligningOrigin")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiCircleRotorAligningOrigin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e41 = None;
                let mut e42 = None;
                let mut e43 = None;
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut scalar = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiCircleRotorAligningOriginField::e41 => {
                            if e41.is_some() {
                                return Err(serde::de::Error::duplicate_field("e41"));
                            }
                            e41 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e42 => {
                            if e42.is_some() {
                                return Err(serde::de::Error::duplicate_field("e42"));
                            }
                            e42 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e43 => {
                            if e43.is_some() {
                                return Err(serde::de::Error::duplicate_field("e43"));
                            }
                            e43 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiCircleRotorAligningOrigin::from([0.0; 10]);
                result[crate::elements::e41] = e41.ok_or_else(|| serde::de::Error::missing_field("e41"))?;
                result[crate::elements::e42] = e42.ok_or_else(|| serde::de::Error::missing_field("e42"))?;
                result[crate::elements::e43] = e43.ok_or_else(|| serde::de::Error::missing_field("e43"))?;
                result[crate::elements::e23] = e23.ok_or_else(|| serde::de::Error::missing_field("e23"))?;
                result[crate::elements::e31] = e31.ok_or_else(|| serde::de::Error::missing_field("e31"))?;
                result[crate::elements::e12] = e12.ok_or_else(|| serde::de::Error::missing_field("e12"))?;
                result[crate::elements::e15] = e15.ok_or_else(|| serde::de::Error::missing_field("e15"))?;
                result[crate::elements::e25] = e25.ok_or_else(|| serde::de::Error::missing_field("e25"))?;
                result[crate::elements::e35] = e35.ok_or_else(|| serde::de::Error::missing_field("e35"))?;
                result[crate::elements::scalar] = scalar.ok_or_else(|| serde::de::Error::missing_field("scalar"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e41", "e42", "e43", "e23", "e31", "e12", "e15", "e25", "e35", "scalar"];
        deserializer.deserialize_struct("AntiCircleRotorAligningOrigin", FIELDS, AntiCircleRotorAligningOriginVisitor)
    }
}
impl std::ops::Index<crate::elements::e41> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e41) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e42> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e42) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e43> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e43) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e23> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e15> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e25> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e35> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiCircleRotorAligningOrigin {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e41> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e41) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e42> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e42) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e43> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e43) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiCircleRotorAligningOrigin {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[9]
    }
}
include!("./impls/anti_circle_rotor_aligning_origin.rs");
