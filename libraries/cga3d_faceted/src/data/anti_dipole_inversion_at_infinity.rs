use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiDipoleInversionAtInfinity.
/// This variant of VersorEvenAtInfinity is the Dual to DipoleInversionAtInfinity. It is common for
/// objects of this type to not intersect the null cone, which also prevents them from
/// projecting onto the horosphere in the usual manner. When this happens, this
/// object has behavioral and operative similarity to a VersorEvenAtInfinity,
/// but an imaginary radius, and a spacial presence in the shape of a
/// DipoleInversionAtInfinity with a real radius.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiDipoleInversionAtInfinity {
    groups: AntiDipoleInversionAtInfinityGroups,
    /// e415, e425, e435, e321, e235, e315, e125, 0, e1, e2, e3, e5
    elements: [f32; 12],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiDipoleInversionAtInfinityGroups {
    /// e415, e425, e435, e321
    g0: Simd32x4,
    /// e235, e315, e125
    g1: Simd32x3,
    /// e1, e2, e3, e5
    g2: Simd32x4,
}
impl AntiDipoleInversionAtInfinity {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(e415: f32, e425: f32, e435: f32, e321: f32, e235: f32, e315: f32, e125: f32, e1: f32, e2: f32, e3: f32, e5: f32) -> Self {
        Self {
            elements: [e415, e425, e435, e321, e235, e315, e125, 0.0, e1, e2, e3, e5],
        }
    }
    pub const fn from_groups(g0: Simd32x4, g1: Simd32x3, g2: Simd32x4) -> Self {
        Self {
            groups: AntiDipoleInversionAtInfinityGroups { g0, g1, g2 },
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
const ANTI_DIPOLE_INVERSION_AT_INFINITY_INDEX_REMAP: [usize; 11] = [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11];
impl std::ops::Index<usize> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_DIPOLE_INVERSION_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_DIPOLE_INVERSION_AT_INFINITY_INDEX_REMAP[index]] }
    }
}
impl From<AntiDipoleInversionAtInfinity> for [f32; 11] {
    fn from(vector: AntiDipoleInversionAtInfinity) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[3], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[8],
                vector.elements[9], vector.elements[10], vector.elements[11],
            ]
        }
    }
}
impl From<[f32; 11]> for AntiDipoleInversionAtInfinity {
    fn from(array: [f32; 11]) -> Self {
        Self {
            elements: [array[0], array[1], array[2], array[3], array[1], array[2], array[3], 0.0, array[2], array[3], array[4], array[5]],
        }
    }
}
impl std::fmt::Debug for AntiDipoleInversionAtInfinity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiDipoleInversionAtInfinity")
            .field("e415", &self[0])
            .field("e425", &self[1])
            .field("e435", &self[2])
            .field("e321", &self[3])
            .field("e235", &self[4])
            .field("e315", &self[5])
            .field("e125", &self[6])
            .field("e1", &self[7])
            .field("e2", &self[8])
            .field("e3", &self[9])
            .field("e5", &self[10])
            .finish()
    }
}

impl AntiDipoleInversionAtInfinity {
    pub const LEN: usize = 11;
}

impl nearly::NearlyEqEps<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {
    fn nearly_eq_eps(&self, other: &AntiDipoleInversionAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {
    fn nearly_eq_ulps(&self, other: &AntiDipoleInversionAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {}
impl nearly::NearlyEq<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {}
impl nearly::NearlyOrdUlps<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {
    fn nearly_lt_ulps(&self, other: &AntiDipoleInversionAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiDipoleInversionAtInfinity, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {
    fn nearly_lt_eps(&self, other: &AntiDipoleInversionAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiDipoleInversionAtInfinity, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {}
impl nearly::NearlyOrd<AntiDipoleInversionAtInfinity, f32, f32> for AntiDipoleInversionAtInfinity {}

impl AntiDipoleInversionAtInfinity {
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

impl PartialOrd for AntiDipoleInversionAtInfinity {
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
impl Ord for AntiDipoleInversionAtInfinity {
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
impl PartialEq for AntiDipoleInversionAtInfinity {
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
impl Eq for AntiDipoleInversionAtInfinity {}
impl std::hash::Hash for AntiDipoleInversionAtInfinity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiDipoleInversionAtInfinity {}
unsafe impl bytemuck::Pod for AntiDipoleInversionAtInfinity {}
impl encase::ShaderType for AntiDipoleInversionAtInfinity {
    type ExtraMetadata = <AntiDipoleInversionAtInfinityGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiDipoleInversionAtInfinityGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiDipoleInversionAtInfinityGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiDipoleInversionAtInfinityGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiDipoleInversionAtInfinityGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiDipoleInversionAtInfinity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiDipoleInversionAtInfinity", 11)?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiDipoleInversionAtInfinity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiDipoleInversionAtInfinityField {
            e415,
            e425,
            e435,
            e321,
            e235,
            e315,
            e125,
            e1,
            e2,
            e3,
            e5,
        }
        struct AntiDipoleInversionAtInfinityVisitor;
        impl<'de> Visitor<'de> for AntiDipoleInversionAtInfinityVisitor {
            type Value = AntiDipoleInversionAtInfinity;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiDipoleInversionAtInfinity")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiDipoleInversionAtInfinity, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiDipoleInversionAtInfinityField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionAtInfinityField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiDipoleInversionAtInfinity::from([0.0; 11]);
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e415", "e425", "e435", "e321", "e235", "e315", "e125", "e1", "e2", "e3", "e5"];
        deserializer.deserialize_struct("AntiDipoleInversionAtInfinity", FIELDS, AntiDipoleInversionAtInfinityVisitor)
    }
}
impl std::ops::Index<crate::elements::e415> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e425> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e435> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e235> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e315> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e125> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e5> for AntiDipoleInversionAtInfinity {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for AntiDipoleInversionAtInfinity {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[10]
    }
}
include!("./impls/anti_dipole_inversion_at_infinity.rs");
