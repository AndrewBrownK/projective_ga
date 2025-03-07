use crate::data::*;
#[allow(unused_imports)]
use crate::simd::*;

/// AntiDipoleInversion
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntiDipoleInversion {
    groups: AntiDipoleInversionGroups,
    /// e423, e431, e412, 0, e415, e425, e435, e321, e235, e315, e125, e4, e1, e2, e3, e5
    elements: [f32; 16],
}
#[repr(C)]
#[derive(Clone, Copy, encase::ShaderType)]
pub struct AntiDipoleInversionGroups {
    /// e423, e431, e412
    g0: Simd32x3,
    /// e415, e425, e435, e321
    g1: Simd32x4,
    /// e235, e315, e125, e4
    g2: Simd32x4,
    /// e1, e2, e3, e5
    g3: Simd32x4,
}
impl AntiDipoleInversion {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_elements(
        e423: f32,
        e431: f32,
        e412: f32,
        e415: f32,
        e425: f32,
        e435: f32,
        e321: f32,
        e235: f32,
        e315: f32,
        e125: f32,
        e4: f32,
        e1: f32,
        e2: f32,
        e3: f32,
        e5: f32,
    ) -> Self {
        Self {
            elements: [e423, e431, e412, 0.0, e415, e425, e435, e321, e235, e315, e125, e4, e1, e2, e3, e5],
        }
    }
    pub const fn from_groups(g0: Simd32x3, g1: Simd32x4, g2: Simd32x4, g3: Simd32x4) -> Self {
        Self {
            groups: AntiDipoleInversionGroups { g0, g1, g2, g3 },
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
    #[inline(always)]
    pub fn group2(&self) -> Simd32x4 {
        unsafe { self.groups.g2 }
    }
    #[inline(always)]
    pub fn group2_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g2 }
    }
    #[inline(always)]
    pub fn group3(&self) -> Simd32x4 {
        unsafe { self.groups.g3 }
    }
    #[inline(always)]
    pub fn group3_mut(&mut self) -> &mut Simd32x4 {
        unsafe { &mut self.groups.g3 }
    }
}
const ANTI_DIPOLE_INVERSION_INDEX_REMAP: [usize; 15] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
impl std::ops::Index<usize> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.elements[ANTI_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl std::ops::IndexMut<usize> for AntiDipoleInversion {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut self.elements[ANTI_DIPOLE_INVERSION_INDEX_REMAP[index]] }
    }
}
impl From<AntiDipoleInversion> for [f32; 15] {
    fn from(vector: AntiDipoleInversion) -> Self {
        unsafe {
            [
                vector.elements[0], vector.elements[1], vector.elements[2], vector.elements[4], vector.elements[5], vector.elements[6], vector.elements[7], vector.elements[8],
                vector.elements[9], vector.elements[10], vector.elements[11], vector.elements[12], vector.elements[13], vector.elements[14], vector.elements[15],
            ]
        }
    }
}
impl From<[f32; 15]> for AntiDipoleInversion {
    fn from(array: [f32; 15]) -> Self {
        Self {
            elements: [
                array[0], array[1], array[2], 0.0, array[1], array[2], array[3], array[4], array[2], array[3], array[4], array[5], array[3], array[4], array[5], array[6],
            ],
        }
    }
}
impl std::fmt::Debug for AntiDipoleInversion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .debug_struct("AntiDipoleInversion")
            .field("e423", &self[0])
            .field("e431", &self[1])
            .field("e412", &self[2])
            .field("e415", &self[3])
            .field("e425", &self[4])
            .field("e435", &self[5])
            .field("e321", &self[6])
            .field("e235", &self[7])
            .field("e315", &self[8])
            .field("e125", &self[9])
            .field("e4", &self[10])
            .field("e1", &self[11])
            .field("e2", &self[12])
            .field("e3", &self[13])
            .field("e5", &self[14])
            .finish()
    }
}

impl AntiDipoleInversion {
    pub const LEN: usize = 15;
}

impl nearly::NearlyEqEps<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {
    fn nearly_eq_eps(&self, other: &AntiDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqUlps<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {
    fn nearly_eq_ulps(&self, other: &AntiDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyEqTol<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {}
impl nearly::NearlyEq<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {}
impl nearly::NearlyOrdUlps<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {
    fn nearly_lt_ulps(&self, other: &AntiDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_ulps(&self, other: &AntiDipoleInversion, ulps: &nearly::UlpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdEps<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {
    fn nearly_lt_eps(&self, other: &AntiDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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

    fn nearly_gt_eps(&self, other: &AntiDipoleInversion, eps: &nearly::EpsToleranceType<f32, f32>) -> bool {
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
impl nearly::NearlyOrdTol<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {}
impl nearly::NearlyOrd<AntiDipoleInversion, f32, f32> for AntiDipoleInversion {}

impl AntiDipoleInversion {
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

impl PartialOrd for AntiDipoleInversion {
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
impl Ord for AntiDipoleInversion {
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
impl PartialEq for AntiDipoleInversion {
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
impl Eq for AntiDipoleInversion {}
impl std::hash::Hash for AntiDipoleInversion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for i in 0..Self::LEN {
            self[i].to_bits().hash(state);
        }
    }
}

unsafe impl bytemuck::Zeroable for AntiDipoleInversion {}
unsafe impl bytemuck::Pod for AntiDipoleInversion {}
impl encase::ShaderType for AntiDipoleInversion {
    type ExtraMetadata = <AntiDipoleInversionGroups as encase::ShaderType>::ExtraMetadata;
    const METADATA: encase::private::Metadata<Self::ExtraMetadata> = <AntiDipoleInversionGroups as encase::ShaderType>::METADATA;
    fn min_size() -> std::num::NonZeroU64 {
        <AntiDipoleInversionGroups as encase::ShaderType>::min_size()
    }
    fn size(&self) -> std::num::NonZeroU64 {
        encase::ShaderType::size(unsafe { &self.groups })
    }
    const UNIFORM_COMPAT_ASSERT: fn() = <AntiDipoleInversionGroups as encase::ShaderType>::UNIFORM_COMPAT_ASSERT;
    fn assert_uniform_compat() {
        <AntiDipoleInversionGroups as encase::ShaderType>::assert_uniform_compat()
    }
}

impl serde::Serialize for AntiDipoleInversion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AntiDipoleInversion", 15)?;
        state.serialize_field("e423", &self[crate::elements::e423])?;
        state.serialize_field("e431", &self[crate::elements::e431])?;
        state.serialize_field("e412", &self[crate::elements::e412])?;
        state.serialize_field("e415", &self[crate::elements::e415])?;
        state.serialize_field("e425", &self[crate::elements::e425])?;
        state.serialize_field("e435", &self[crate::elements::e435])?;
        state.serialize_field("e321", &self[crate::elements::e321])?;
        state.serialize_field("e235", &self[crate::elements::e235])?;
        state.serialize_field("e315", &self[crate::elements::e315])?;
        state.serialize_field("e125", &self[crate::elements::e125])?;
        state.serialize_field("e4", &self[crate::elements::e4])?;
        state.serialize_field("e1", &self[crate::elements::e1])?;
        state.serialize_field("e2", &self[crate::elements::e2])?;
        state.serialize_field("e3", &self[crate::elements::e3])?;
        state.serialize_field("e5", &self[crate::elements::e5])?;
        state.end()
    }
}
impl<'de> serde::Deserialize<'de> for AntiDipoleInversion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;
        #[allow(non_camel_case_types)]
        #[derive(serde::Deserialize)]
        enum AntiDipoleInversionField {
            e423,
            e431,
            e412,
            e415,
            e425,
            e435,
            e321,
            e235,
            e315,
            e125,
            e4,
            e1,
            e2,
            e3,
            e5,
        }
        struct AntiDipoleInversionVisitor;
        impl<'de> Visitor<'de> for AntiDipoleInversionVisitor {
            type Value = AntiDipoleInversion;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AntiDipoleInversion")
            }
            fn visit_map<V>(self, mut map: V) -> Result<AntiDipoleInversion, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut e423 = None;
                let mut e431 = None;
                let mut e412 = None;
                let mut e415 = None;
                let mut e425 = None;
                let mut e435 = None;
                let mut e321 = None;
                let mut e235 = None;
                let mut e315 = None;
                let mut e125 = None;
                let mut e4 = None;
                let mut e1 = None;
                let mut e2 = None;
                let mut e3 = None;
                let mut e5 = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        AntiDipoleInversionField::e423 => {
                            if e423.is_some() {
                                return Err(serde::de::Error::duplicate_field("e423"));
                            }
                            e423 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e431 => {
                            if e431.is_some() {
                                return Err(serde::de::Error::duplicate_field("e431"));
                            }
                            e431 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e412 => {
                            if e412.is_some() {
                                return Err(serde::de::Error::duplicate_field("e412"));
                            }
                            e412 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e415 => {
                            if e415.is_some() {
                                return Err(serde::de::Error::duplicate_field("e415"));
                            }
                            e415 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e425 => {
                            if e425.is_some() {
                                return Err(serde::de::Error::duplicate_field("e425"));
                            }
                            e425 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e435 => {
                            if e435.is_some() {
                                return Err(serde::de::Error::duplicate_field("e435"));
                            }
                            e435 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e321 => {
                            if e321.is_some() {
                                return Err(serde::de::Error::duplicate_field("e321"));
                            }
                            e321 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e235 => {
                            if e235.is_some() {
                                return Err(serde::de::Error::duplicate_field("e235"));
                            }
                            e235 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e315 => {
                            if e315.is_some() {
                                return Err(serde::de::Error::duplicate_field("e315"));
                            }
                            e315 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e125 => {
                            if e125.is_some() {
                                return Err(serde::de::Error::duplicate_field("e125"));
                            }
                            e125 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e4 => {
                            if e4.is_some() {
                                return Err(serde::de::Error::duplicate_field("e4"));
                            }
                            e4 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e1 => {
                            if e1.is_some() {
                                return Err(serde::de::Error::duplicate_field("e1"));
                            }
                            e1 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e2 => {
                            if e2.is_some() {
                                return Err(serde::de::Error::duplicate_field("e2"));
                            }
                            e2 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e3 => {
                            if e3.is_some() {
                                return Err(serde::de::Error::duplicate_field("e3"));
                            }
                            e3 = Some(map.next_value()?);
                        }

                        AntiDipoleInversionField::e5 => {
                            if e5.is_some() {
                                return Err(serde::de::Error::duplicate_field("e5"));
                            }
                            e5 = Some(map.next_value()?);
                        }
                    }
                }
                let mut result = AntiDipoleInversion::from([0.0; 15]);
                result[crate::elements::e423] = e423.ok_or_else(|| serde::de::Error::missing_field("e423"))?;
                result[crate::elements::e431] = e431.ok_or_else(|| serde::de::Error::missing_field("e431"))?;
                result[crate::elements::e412] = e412.ok_or_else(|| serde::de::Error::missing_field("e412"))?;
                result[crate::elements::e415] = e415.ok_or_else(|| serde::de::Error::missing_field("e415"))?;
                result[crate::elements::e425] = e425.ok_or_else(|| serde::de::Error::missing_field("e425"))?;
                result[crate::elements::e435] = e435.ok_or_else(|| serde::de::Error::missing_field("e435"))?;
                result[crate::elements::e321] = e321.ok_or_else(|| serde::de::Error::missing_field("e321"))?;
                result[crate::elements::e235] = e235.ok_or_else(|| serde::de::Error::missing_field("e235"))?;
                result[crate::elements::e315] = e315.ok_or_else(|| serde::de::Error::missing_field("e315"))?;
                result[crate::elements::e125] = e125.ok_or_else(|| serde::de::Error::missing_field("e125"))?;
                result[crate::elements::e4] = e4.ok_or_else(|| serde::de::Error::missing_field("e4"))?;
                result[crate::elements::e1] = e1.ok_or_else(|| serde::de::Error::missing_field("e1"))?;
                result[crate::elements::e2] = e2.ok_or_else(|| serde::de::Error::missing_field("e2"))?;
                result[crate::elements::e3] = e3.ok_or_else(|| serde::de::Error::missing_field("e3"))?;
                result[crate::elements::e5] = e5.ok_or_else(|| serde::de::Error::missing_field("e5"))?;
                Ok(result)
            }
        }

        const FIELDS: &'static [&'static str] = &["e423", "e431", "e412", "e415", "e425", "e435", "e321", "e235", "e315", "e125", "e4", "e1", "e2", "e3", "e5"];
        deserializer.deserialize_struct("AntiDipoleInversion", FIELDS, AntiDipoleInversionVisitor)
    }
}
impl std::ops::Index<crate::elements::e423> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e423) -> &Self::Output {
        &self[0]
    }
}
impl std::ops::Index<crate::elements::e431> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e431) -> &Self::Output {
        &self[1]
    }
}
impl std::ops::Index<crate::elements::e412> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e412) -> &Self::Output {
        &self[2]
    }
}
impl std::ops::Index<crate::elements::e415> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e415) -> &Self::Output {
        &self[3]
    }
}
impl std::ops::Index<crate::elements::e425> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e425) -> &Self::Output {
        &self[4]
    }
}
impl std::ops::Index<crate::elements::e435> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e435) -> &Self::Output {
        &self[5]
    }
}
impl std::ops::Index<crate::elements::e321> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e321) -> &Self::Output {
        &self[6]
    }
}
impl std::ops::Index<crate::elements::e235> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e235) -> &Self::Output {
        &self[7]
    }
}
impl std::ops::Index<crate::elements::e315> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e315) -> &Self::Output {
        &self[8]
    }
}
impl std::ops::Index<crate::elements::e125> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e125) -> &Self::Output {
        &self[9]
    }
}
impl std::ops::Index<crate::elements::e4> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e4) -> &Self::Output {
        &self[10]
    }
}
impl std::ops::Index<crate::elements::e1> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e1) -> &Self::Output {
        &self[11]
    }
}
impl std::ops::Index<crate::elements::e2> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e2) -> &Self::Output {
        &self[12]
    }
}
impl std::ops::Index<crate::elements::e3> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e3) -> &Self::Output {
        &self[13]
    }
}
impl std::ops::Index<crate::elements::e5> for AntiDipoleInversion {
    type Output = f32;
    fn index(&self, _: crate::elements::e5) -> &Self::Output {
        &self[14]
    }
}
impl std::ops::IndexMut<crate::elements::e423> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e423) -> &mut Self::Output {
        &mut self[0]
    }
}
impl std::ops::IndexMut<crate::elements::e431> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e431) -> &mut Self::Output {
        &mut self[1]
    }
}
impl std::ops::IndexMut<crate::elements::e412> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e412) -> &mut Self::Output {
        &mut self[2]
    }
}
impl std::ops::IndexMut<crate::elements::e415> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e415) -> &mut Self::Output {
        &mut self[3]
    }
}
impl std::ops::IndexMut<crate::elements::e425> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e425) -> &mut Self::Output {
        &mut self[4]
    }
}
impl std::ops::IndexMut<crate::elements::e435> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e435) -> &mut Self::Output {
        &mut self[5]
    }
}
impl std::ops::IndexMut<crate::elements::e321> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e321) -> &mut Self::Output {
        &mut self[6]
    }
}
impl std::ops::IndexMut<crate::elements::e235> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e235) -> &mut Self::Output {
        &mut self[7]
    }
}
impl std::ops::IndexMut<crate::elements::e315> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e315) -> &mut Self::Output {
        &mut self[8]
    }
}
impl std::ops::IndexMut<crate::elements::e125> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e125) -> &mut Self::Output {
        &mut self[9]
    }
}
impl std::ops::IndexMut<crate::elements::e4> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e4) -> &mut Self::Output {
        &mut self[10]
    }
}
impl std::ops::IndexMut<crate::elements::e1> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e1) -> &mut Self::Output {
        &mut self[11]
    }
}
impl std::ops::IndexMut<crate::elements::e2> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e2) -> &mut Self::Output {
        &mut self[12]
    }
}
impl std::ops::IndexMut<crate::elements::e3> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e3) -> &mut Self::Output {
        &mut self[13]
    }
}
impl std::ops::IndexMut<crate::elements::e5> for AntiDipoleInversion {
    fn index_mut(&mut self, _: crate::elements::e5) -> &mut Self::Output {
        &mut self[14]
    }
}
include!("./impls/anti_dipole_inversion.rs");
