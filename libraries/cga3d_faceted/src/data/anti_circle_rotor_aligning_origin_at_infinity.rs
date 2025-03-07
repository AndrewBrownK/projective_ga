use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiCircleRotorAligningOriginAtInfinity.
/// This variant of VersorOddOrthogonalOrigin is the Dual to CircleRotorAligningOriginAtInfinity. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorOddOrthogonalOrigin,
/// but an imaginary radius, and a spacial presence in the shape of a
/// CircleRotorAligningOriginAtInfinity with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiCircleRotorAligningOriginAtInfinity {
    groups: AntiCircleRotorAligningOriginAtInfinityGroups,
    /// e23, e31, e12, 0, e15, e25, e35, scalar
    elements: [f32; 8],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiCircleRotorAligningOriginAtInfinityGroups {
    /// e23, e31, e12
    g0: Simd32x3,
    /// e15, e25, e35, scalar
    g1: Simd32x4,
}
impl AntiCircleRotorAligningOriginAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e23: f32, e31: f32, e12: f32, e15: f32, e25: f32, e35: f32, scalar: f32) -> Self {
        Self {
            elements: [e23, e31, e12, 0.0, e15, e25, e35, scalar],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4) -> Self {
        Self {
            groups: AntiCircleRotorAligningOriginAtInfinityGroups { g0, g1 },
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
const ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP: [usize; 7] = [0, 1, 2, 4, 5, 6, 7];
impl std::ops::Index<usize> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_CIRCLE_ROTOR_ALIGNING_ORIGIN_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<AntiCircleRotorAligningOriginAtInfinity> for [f32; 7] {
    fn from(vector: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7],
            ]
        }
    }
}
impl From<[f32; 7]> for AntiCircleRotorAligningOriginAtInfinity {
    fn from(array: [f32; 7]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4]],
        }
    }
}
impl std::fmt::Debug for AntiCircleRotorAligningOriginAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiCircleRotorAligningOriginAtInfinity")
            .field("e23", &self[0])
            .field("e31", &self[1])
            .field("e12", &self[2])
            .field("e15", &self[3])
            .field("e25", &self[4])
            .field("e35", &self[5])
            .field("scalar", &self[6])
            .finish()
    }
}

impl AntiCircleRotorAligningOriginAtInfinity {
    pub const LEN: usize = 7;
}

impl nearly::NearlyEqEps<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {
    fn nearly_eq_eps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {
    fn nearly_eq_ulps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyEq<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyOrdUlps<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {
    fn nearly_lt_ulps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {
    fn nearly_lt_eps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiCircleRotorAligningOriginAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {}
impl nearly::NearlyOrd<AntiCircleRotorAligningOriginAtInfinity, f32, f32> for AntiCircleRotorAligningOriginAtInfinity {}

impl AntiCircleRotorAligningOriginAtInfinity {
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

impl PartialOrd for AntiCircleRotorAligningOriginAtInfinity {
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
impl Ord for AntiCircleRotorAligningOriginAtInfinity {
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
impl PartialEq for AntiCircleRotorAligningOriginAtInfinity {
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
impl Eq for AntiCircleRotorAligningOriginAtInfinity {}
impl std::hash::Hash for AntiCircleRotorAligningOriginAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiCircleRotorAligningOriginAtInfinity {}
unsafe impl bytemuck::Pod for AntiCircleRotorAligningOriginAtInfinity {}
impl encase::ShaderType for AntiCircleRotorAligningOriginAtInfinity {
    type ExtraMetadata = <AntiCircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiCircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiCircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiCircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiCircleRotorAligningOriginAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiCircleRotorAligningOriginAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiCircleRotorAligningOriginAtInfinity", 7)?;
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
impl<'de> serde::Deserialize<'de> for AntiCircleRotorAligningOriginAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiCircleRotorAligningOriginAtInfinityField {
            e23,
            e31,
            e12,
            e15,
            e25,
            e35,
            scalar,
        }
        struct AntiCircleRotorAligningOriginAtInfinityVisitor;
        impl<'de> Visitor<'de> for AntiCircleRotorAligningOriginAtInfinityVisitor {
            type Value = AntiCircleRotorAligningOriginAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiCircleRotorAligningOriginAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiCircleRotorAligningOriginAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e23 = None;
                let mut e31 = None;
                let mut e12 = None;
                let mut e15 = None;
                let mut e25 = None;
                let mut e35 = None;
                let mut scalar = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiCircleRotorAligningOriginAtInfinityField::e23 => {
                            if e23.is_some() {
                                return Err(serde::de::Error::duplicate_field("e23"));
                            }
                            e23 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::e31 => {
                            if e31.is_some() {
                                return Err(serde::de::Error::duplicate_field("e31"));
                            }
                            e31 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::e12 => {
                            if e12.is_some() {
                                return Err(serde::de::Error::duplicate_field("e12"));
                            }
                            e12 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::e15 => {
                            if e15.is_some() {
                                return Err(serde::de::Error::duplicate_field("e15"));
                            }
                            e15 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::e25 => {
                            if e25.is_some() {
                                return Err(serde::de::Error::duplicate_field("e25"));
                            }
                            e25 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::e35 => {
                            if e35.is_some() {
                                return Err(serde::de::Error::duplicate_field("e35"));
                            }
                            e35 = Some(map.next_value()?);
                        }

                        AntiCircleRotorAligningOriginAtInfinityField::scalar => {
                            if scalar.is_some() {
                                return Err(serde::de::Error::duplicate_field("scalar"));
                            }
                            scalar = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiCircleRotorAligningOriginAtInfinity::from([0.0; 7]);
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

        const FIELDS: &'static [&'static str] = &["e23", "e31", "e12", "e15", "e25", "e35", "scalar"];
        deserializer.deserialize_struct("AntiCircleRotorAligningOriginAtInfinity", FIELDS, AntiCircleRotorAligningOriginAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e23> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e23) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e31> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e31) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e12> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e12) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e15> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e15) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e25> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e25) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e35> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e35) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::scalar> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::scalar) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e23> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e23) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e31> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e31) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e12> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e12) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e15> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e15) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e25> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e25) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e35> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e35) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::scalar> for AntiCircleRotorAligningOriginAtInfinity {
    fn index_mut(&mut self, _: crate::elements::scalar) -> &mut Self::Output {
        &mut self[6]
    }
}
include!("./impls/anti_circle_rotor_aligning_origin_at_infinity.rs");
